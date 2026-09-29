//! Resolve the intra-doc links that doxygen generated from C names.
//!
//! The C headers refer to other items by their C name, but the bindings are
//! split across modules and crates, enumerators become associated constants,
//! and some items are renamed. Once all bindings are written, index the public
//! items of every crate in the workspace and rewrite each `` [`C_NAME`] `` link
//! in the generated files to `` [`C_NAME`](crate::path::to::RustItem) ``.
//!
//! Links that can't be resolved are demoted to plain code spans, unless
//! `KEEP_UNRESOLVED_DOC_LINKS` is set. Keeping them lets rustdoc report each
//! one, to check whether it should have been resolved.

use crate::c_names::{CNames, C_NAMES};
use anyhow::{bail, Context};
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// `(C name, Rust name)` of C items which have no public binding, but are
/// replaced by a hand-written public item.
const OVERRIDES: &[(&str, &str)] = &[
    (
        "OH_NATIVEXCOMPONENT_RESULT_SUCCESS",
        "XcomponentResult::SUCCESS",
    ),
    (
        "OH_NATIVEXCOMPONENT_RESULT_FAILED",
        "XcomponentResult::FAILED",
    ),
    (
        "OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER",
        "XcomponentResult::BAD_PARAMETER",
    ),
];

pub(crate) fn resolve_doc_links(
    root_dir: &Path,
    files: &[PathBuf],
    keep_unresolved: bool,
) -> anyhow::Result<()> {
    let crates = find_crates(root_dir)?;
    let mut modules = HashMap::new();
    let mut index = Index::default();
    for (krate, c) in crates.iter().enumerate() {
        let files = module_files(&c.lib_rs)?;
        for module in &files {
            index.add_items(krate, &module.path, module.is_public, &module.ast.items);
        }
        for module in files {
            if module.is_public {
                for reexport in named_reexports(&module.ast) {
                    index.add_reexport(krate, &module.path, &reexport);
                }
                for source in crate_glob_reexports(&module.ast) {
                    index.add_glob_reexport(krate, &module.path, &source);
                }
            }
            modules.insert(module.file, (krate, module.path));
        }
    }
    let c_names = C_NAMES.lock().unwrap();
    let resolver = Resolver {
        crates: &crates,
        index: &index,
        c_names: &c_names,
    };

    let mut resolved = 0;
    let mut unresolved = 0;
    let mut seen = HashSet::new();
    for file in files {
        let file = file.canonicalize()?;
        if !seen.insert(file.clone()) {
            continue;
        }
        let Some((krate, module)) = modules.get(&file) else {
            warn!(
                "{} is not part of any crate's module tree, not resolving its doc links",
                file.display()
            );
            continue;
        };
        let source = fs::read_to_string(&file)?;
        let mut lines = Vec::new();
        for line in source.split('\n') {
            if !line.trim_start().starts_with("///") && !line.trim_start().starts_with("//!") {
                lines.push(line.to_string());
                continue;
            }
            lines.push(rewrite_links(line, keep_unresolved, |name| {
                let link = resolver.resolve(name, *krate, module);
                if link.is_some() {
                    resolved += 1;
                } else {
                    unresolved += 1;
                    debug!("Unresolved doc link `{name}` in {}", file.display());
                }
                link
            }));
        }
        let rewritten = lines.join("\n");
        if rewritten != source {
            fs::write(&file, rewritten)?;
        }
    }
    let action = if keep_unresolved {
        "kept"
    } else {
        "demoted to code spans"
    };
    info!("Resolved {resolved} doc links, {unresolved} unresolved links {action}");
    Ok(())
}

/// Replace every `` [`name`] `` link in `line` with a link to the destination
/// that `resolve` returns for `name`. Links which already have a destination
/// are left alone.
fn rewrite_links(
    line: &str,
    keep_unresolved: bool,
    mut resolve: impl FnMut(&str) -> Option<String>,
) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find("[`") {
        let (before, link) = rest.split_at(start);
        out.push_str(before);
        let Some(end) = link[2..].find("`]").map(|end| end + 2) else {
            out.push_str(link);
            return out;
        };
        let name = &link[2..end];
        let after = &link[end + 2..];
        let is_shortcut_link = !name.is_empty()
            && !name.contains(['`', '[', ']'])
            && !after.starts_with(['(', '['])
            && !before.ends_with([']', '\\']);
        if !is_shortcut_link {
            out.push_str(&link[..end + 2]);
            rest = after;
            continue;
        }
        // Doxygen refers to members as `#member` or `Type#member`.
        let name = name.trim_start_matches('#');
        match resolve(name) {
            Some(dest) => out.push_str(&format!("[`{name}`]({dest})")),
            None if keep_unresolved => out.push_str(&link[..end + 2]),
            None => out.push_str(&format!("`{name}`")),
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

struct Crate {
    /// The crate name, as used in paths.
    name: String,
    /// The crate names of the dependencies.
    dependencies: Vec<String>,
    lib_rs: PathBuf,
}

/// Find the root crate and all crates below `components`.
fn find_crates(root_dir: &Path) -> anyhow::Result<Vec<Crate>> {
    let mut manifests = vec![root_dir.join("Cargo.toml")];
    let mut dirs = vec![root_dir.join("components")];
    while let Some(dir) = dirs.pop() {
        for entry in
            fs::read_dir(&dir).with_context(|| format!("Failed to read {}", dir.display()))?
        {
            let path = entry?.path();
            if path.is_dir() && !path.ends_with("target") {
                dirs.push(path);
            } else if path.file_name().is_some_and(|name| name == "Cargo.toml") {
                manifests.push(path);
            }
        }
    }
    let mut crates = vec![];
    for manifest in manifests {
        let contents = fs::read_to_string(&manifest)?;
        let Some((name, dependencies)) = parse_manifest(&contents) else {
            continue;
        };
        let lib_rs = manifest.with_file_name("src/lib.rs");
        if lib_rs.exists() {
            crates.push(Crate {
                name,
                dependencies,
                lib_rs,
            });
        }
    }
    crates.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(crates)
}

/// Return the crate name and the crate names of the `[dependencies]`, or
/// `None` if the manifest has no `[package]`.
///
/// The manifests in this repository are simple enough to not need a TOML
/// parser.
fn parse_manifest(manifest: &str) -> Option<(String, Vec<String>)> {
    let mut section = "";
    let mut name = None;
    let mut dependencies = vec![];
    for line in manifest.lines().map(str::trim) {
        if let Some(header) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = header;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.starts_with('#') {
            continue;
        }
        match section {
            "package" if key == "name" => {
                name = Some(value.trim().trim_matches('"').replace('-', "_"));
            }
            "dependencies" => dependencies.push(key.replace('-', "_")),
            _ => {}
        }
    }
    Some((name?, dependencies))
}

struct ModuleFile {
    file: PathBuf,
    /// Path of the module the items of the file are visible at.
    path: Vec<String>,
    /// Whether the items are reachable from outside the crate.
    is_public: bool,
    ast: syn::File,
}

/// Parse all module files of the crate rooted at `lib_rs`.
fn module_files(lib_rs: &Path) -> anyhow::Result<Vec<ModuleFile>> {
    let mut modules = vec![];
    let mut pending = vec![(lib_rs.canonicalize()?, true, vec![], true)];
    while let Some((file, is_mod_rs, path, is_public)) = pending.pop() {
        let source = fs::read_to_string(&file)?;
        let ast = syn::parse_file(&source)
            .with_context(|| format!("Failed to parse {}", file.display()))?;
        let dir = file.parent().context("No parent directory")?;
        let child_dir = if is_mod_rs {
            dir.to_path_buf()
        } else {
            dir.join(file.file_stem().context("No file stem")?)
        };
        let glob_reexports: HashSet<String> = ast
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Use(u) if is_pub(&u.vis) => match &u.tree {
                    syn::UseTree::Path(p) if matches!(*p.tree, syn::UseTree::Glob(_)) => {
                        Some(p.ident.to_string())
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();
        for item in &ast.items {
            let syn::Item::Mod(m) = item else { continue };
            if m.content.is_some() {
                continue;
            }
            let name = m.ident.to_string();
            let (child, child_is_mod_rs) = match path_attribute(&m.attrs)? {
                Some(p) => (dir.join(p), true),
                None => {
                    let file = child_dir.join(format!("{name}.rs"));
                    if file.exists() {
                        (file, false)
                    } else {
                        (child_dir.join(&name).join("mod.rs"), true)
                    }
                }
            };
            let Ok(child) = child.canonicalize() else {
                warn!("Module file for `{name}` in {} not found", file.display());
                continue;
            };
            let (child_path, child_is_public) = if is_pub(&m.vis) {
                ([path.clone(), vec![name]].concat(), is_public)
            } else if glob_reexports.contains(&name) {
                (path.clone(), is_public)
            } else {
                ([path.clone(), vec![name]].concat(), false)
            };
            pending.push((child, child_is_mod_rs, child_path, child_is_public));
        }
        modules.push(ModuleFile {
            file,
            path,
            is_public,
            ast,
        });
    }
    Ok(modules)
}

/// A `pub use module::name as alias;` of an item of a child module.
struct Reexport {
    module: String,
    name: String,
    alias: String,
}

fn named_reexports(ast: &syn::File) -> Vec<Reexport> {
    let mut reexports = vec![];
    for item in &ast.items {
        let syn::Item::Use(u) = item else { continue };
        let syn::UseTree::Path(path) = &u.tree else {
            continue;
        };
        if !is_pub(&u.vis) {
            continue;
        }
        let trees = match &*path.tree {
            syn::UseTree::Group(group) => group.items.iter().collect(),
            tree => vec![tree],
        };
        for tree in trees {
            let (name, alias) = match tree {
                syn::UseTree::Name(n) => (&n.ident, &n.ident),
                syn::UseTree::Rename(r) => (&r.ident, &r.rename),
                _ => continue,
            };
            reexports.push(Reexport {
                module: path.ident.to_string(),
                name: name.to_string(),
                alias: alias.to_string(),
            });
        }
    }
    reexports
}

/// Paths of the modules glob re-exported with `pub use crate::path::*;`.
fn crate_glob_reexports(ast: &syn::File) -> Vec<Vec<String>> {
    fn collect(tree: &syn::UseTree, path: &mut Vec<String>, globs: &mut Vec<Vec<String>>) {
        match tree {
            syn::UseTree::Path(p) => {
                path.push(p.ident.to_string());
                collect(&p.tree, path, globs);
                path.pop();
            }
            syn::UseTree::Group(g) => {
                for tree in &g.items {
                    collect(tree, path, globs);
                }
            }
            syn::UseTree::Glob(_) => globs.push(path.clone()),
            _ => {}
        }
    }
    let mut globs = vec![];
    for item in &ast.items {
        let syn::Item::Use(u) = item else { continue };
        match &u.tree {
            syn::UseTree::Path(p) if is_pub(&u.vis) && p.ident == "crate" => {
                collect(&p.tree, &mut vec![], &mut globs);
            }
            _ => {}
        }
    }
    globs
}

fn path_attribute(attrs: &[syn::Attribute]) -> anyhow::Result<Option<String>> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("path")) else {
        return Ok(None);
    };
    match &attr.meta.require_name_value()?.value {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => Ok(Some(s.value())),
        _ => bail!("Unexpected #[path] attribute"),
    }
}

fn is_pub(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

#[derive(Debug)]
struct Target {
    krate: usize,
    module: Vec<String>,
    is_public: bool,
    /// Path relative to the crate root.
    path: String,
    /// The error type, if this is a `Result<(), Error>` alias.
    result_error: Option<String>,
}

#[derive(Default)]
struct Index {
    /// Item name, or `Type::member` for associated items, enum variants,
    /// fields and items of inline modules -> items with that name.
    items: HashMap<String, Vec<Target>>,
}

impl Index {
    fn add(
        &mut self,
        krate: usize,
        module: &[String],
        is_public: bool,
        name: String,
        result_error: Option<String>,
    ) {
        let path = module
            .iter()
            .map(String::as_str)
            .chain(Some(name.as_str()))
            .collect::<Vec<_>>()
            .join("::");
        self.items.entry(name).or_default().push(Target {
            krate,
            module: module.to_vec(),
            is_public,
            path,
            result_error,
        });
    }

    /// Make the item `reexport` refers to, and its members, public in `module`.
    fn add_reexport(&mut self, krate: usize, module: &[String], reexport: &Reexport) {
        let source = [module, std::slice::from_ref(&reexport.module)].concat();
        let mut reexported = vec![];
        for (key, targets) in &self.items {
            let Some(member) = key.strip_prefix(reexport.name.as_str()) else {
                continue;
            };
            if !member.is_empty() && !member.starts_with("::") {
                continue;
            }
            for target in targets
                .iter()
                .filter(|t| t.krate == krate && t.module == source)
            {
                let key = format!("{}{member}", reexport.alias);
                reexported.push((key, target.result_error.clone()));
            }
        }
        for (key, result_error) in reexported {
            self.add(krate, module, true, key, result_error);
        }
    }

    /// Make the non-public items of `source`, which `module` glob re-exports,
    /// public in `module`.
    fn add_glob_reexport(&mut self, krate: usize, module: &[String], source: &[String]) {
        let mut reexported = vec![];
        for (key, targets) in &self.items {
            for target in targets
                .iter()
                .filter(|t| t.krate == krate && t.module == source && !t.is_public)
            {
                reexported.push((key.clone(), target.result_error.clone()));
            }
        }
        for (key, result_error) in reexported {
            self.add(krate, module, true, key, result_error);
        }
    }

    fn add_items(&mut self, krate: usize, module: &[String], is_public: bool, items: &[syn::Item]) {
        use syn::{ForeignItem, ImplItem, Item};
        let mut names = vec![];
        let ident = |ident: &syn::Ident| (ident.to_string(), None);
        for item in items {
            match item {
                Item::ForeignMod(m) => {
                    for item in &m.items {
                        match item {
                            ForeignItem::Fn(f) if is_pub(&f.vis) => names.push(ident(&f.sig.ident)),
                            ForeignItem::Static(s) if is_pub(&s.vis) => names.push(ident(&s.ident)),
                            _ => {}
                        }
                    }
                }
                Item::Fn(f) if is_pub(&f.vis) => names.push(ident(&f.sig.ident)),
                Item::Const(c) if is_pub(&c.vis) => names.push(ident(&c.ident)),
                Item::Static(s) if is_pub(&s.vis) => names.push(ident(&s.ident)),
                Item::Type(t) if is_pub(&t.vis) => {
                    names.push((t.ident.to_string(), result_error_type(&t.ty)))
                }
                Item::Struct(s) if is_pub(&s.vis) => {
                    names.push(ident(&s.ident));
                    let fields = s.fields.iter().filter(|f| is_pub(&f.vis));
                    names.extend(members(&s.ident, fields.filter_map(|f| f.ident.as_ref())));
                }
                Item::Union(u) if is_pub(&u.vis) => {
                    names.push(ident(&u.ident));
                    let fields = u.fields.named.iter().filter(|f| is_pub(&f.vis));
                    names.extend(members(&u.ident, fields.filter_map(|f| f.ident.as_ref())));
                }
                Item::Enum(e) if is_pub(&e.vis) => {
                    names.push(ident(&e.ident));
                    names.extend(members(&e.ident, e.variants.iter().map(|v| &v.ident)));
                }
                Item::Impl(i) if i.trait_.is_none() => {
                    let syn::Type::Path(ty) = &*i.self_ty else {
                        continue;
                    };
                    let Some(owner) = ty.path.segments.last() else {
                        continue;
                    };
                    let items = i.items.iter().filter_map(|item| match item {
                        ImplItem::Const(c) if is_pub(&c.vis) => Some(&c.ident),
                        ImplItem::Fn(f) if is_pub(&f.vis) => Some(&f.sig.ident),
                        _ => None,
                    });
                    names.extend(members(&owner.ident, items));
                }
                // Enums generated with `constified_enum_module`.
                Item::Mod(m) if is_pub(&m.vis) => {
                    let Some((_, items)) = &m.content else {
                        continue;
                    };
                    names.push(ident(&m.ident));
                    let consts = items.iter().filter_map(|item| match item {
                        Item::Const(c) if is_pub(&c.vis) => Some(&c.ident),
                        _ => None,
                    });
                    names.extend(members(&m.ident, consts));
                }
                _ => {}
            }
        }
        for (name, result_error) in names {
            self.add(krate, module, is_public, name, result_error);
        }
    }
}

/// `Owner::member` index keys.
fn members<'a>(
    owner: &'a syn::Ident,
    members: impl IntoIterator<Item = &'a syn::Ident> + 'a,
) -> impl Iterator<Item = (String, Option<String>)> + 'a {
    members
        .into_iter()
        .map(move |member| (format!("{owner}::{member}"), None))
}

/// Return the name of `Error` if `ty` is `Result<(), Error>`.
fn result_error_type(ty: &syn::Type) -> Option<String> {
    let syn::Type::Path(ty) = ty else { return None };
    let segment = ty.path.segments.last()?;
    if segment.ident != "Result" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.iter().nth(1)? {
        syn::GenericArgument::Type(syn::Type::Path(error)) => {
            Some(error.path.segments.last()?.ident.to_string())
        }
        _ => None,
    }
}

struct Resolver<'a> {
    crates: &'a [Crate],
    index: &'a Index,
    c_names: &'a CNames,
}

impl Resolver<'_> {
    /// Resolve the C name `name` to a link destination, as seen from `module`
    /// in crate `krate`.
    ///
    /// Prefers items in the same module, then in the same crate and then in the
    /// crate's dependencies. Items in other crates can't be linked to.
    fn resolve(&self, name: &str, krate: usize, module: &[String]) -> Option<String> {
        let target = self
            .candidates(name)
            .into_iter()
            .filter(|t| t.is_public)
            .filter(|t| {
                t.krate == krate
                    || self.crates[krate]
                        .dependencies
                        .contains(&self.crates[t.krate].name)
            })
            .min_by_key(|t| (t.krate != krate, t.module != module, &t.path, t.krate))?;
        let root = if target.krate == krate {
            "crate"
        } else {
            &self.crates[target.krate].name
        };
        Some(format!("{root}::{}", target.path))
    }

    fn candidates(&self, name: &str) -> Vec<&Target> {
        if let Some((owner, member)) = name.rsplit_once('#').or_else(|| name.rsplit_once("::")) {
            let candidates = self.candidates(member);
            if !candidates.is_empty() {
                return candidates;
            }
            return self.items(&format!("{}::{member}", self.rust_type_name(owner)));
        }
        if let Some((_, rust_name)) = OVERRIDES.iter().find(|(c_name, _)| *c_name == name) {
            return self.items(rust_name);
        }
        let enumerators = self.c_names.enumerators.get(name).into_iter().flatten();
        let candidates: Vec<_> = enumerators
            .flat_map(|e| self.enumerator_candidates(&e.c_enum, &e.rust_name, e.is_zero))
            .collect();
        if !candidates.is_empty() {
            return candidates;
        }
        self.items(self.rust_type_name(name))
    }

    fn enumerator_candidates(&self, c_enum: &str, rust_name: &str, is_zero: bool) -> Vec<&Target> {
        let ty = self.rust_type_name(c_enum);
        let members = self.items(&format!("{ty}::{rust_name}"));
        if !members.is_empty() {
            return members;
        }
        // Result enums are split into a `Result<(), Error>` alias and the
        // `Error` type, which has no constant for the success value.
        let mut candidates = vec![];
        for alias in self.items(ty) {
            let Some(error) = &alias.result_error else {
                continue;
            };
            if is_zero {
                candidates.push(alias);
            } else {
                let members = self.items(&format!("{error}::{rust_name}"));
                candidates.extend(members.into_iter().filter(|t| t.krate == alias.krate));
            }
        }
        candidates
    }

    fn rust_type_name<'b>(&'b self, c_name: &'b str) -> &'b str {
        self.c_names
            .type_renames
            .get(c_name)
            .map_or(c_name, String::as_str)
    }

    fn items(&self, name: &str) -> Vec<&Target> {
        self.index.items.get(name).into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c_names::Enumerator;

    fn resolve_all(line: &str, keep_unresolved: bool) -> String {
        rewrite_links(line, keep_unresolved, |name| {
            (name == "Foo" || name == "Bar#BAZ").then(|| format!("crate::m::{name}"))
        })
    }

    #[test]
    fn rewrites_shortcut_links() {
        assert_eq!(
            resolve_all("/// Returns [`Foo`] or [`Unknown`].", false),
            "/// Returns [`Foo`](crate::m::Foo) or `Unknown`."
        );
        assert_eq!(
            resolve_all("/// Returns [`Foo`] or [`Unknown`].", true),
            "/// Returns [`Foo`](crate::m::Foo) or [`Unknown`]."
        );
    }

    #[test]
    fn rewrites_links_followed_by_colon() {
        assert_eq!(
            resolve_all("/// [`Foo`]: The operation is successful.", false),
            "/// [`Foo`](crate::m::Foo): The operation is successful."
        );
    }

    #[test]
    fn strips_leading_hash() {
        assert_eq!(
            resolve_all("/// [`#Foo`]", false),
            "/// [`Foo`](crate::m::Foo)"
        );
        assert_eq!(
            resolve_all("/// [`Bar#BAZ`]", false),
            "/// [`Bar#BAZ`](crate::m::Bar#BAZ)"
        );
    }

    #[test]
    fn skips_links_with_destination() {
        for line in [
            "/// [`Foo`](crate::Foo)",
            "/// [`Foo`][foo]",
            "/// [text][`Foo`]",
            "/// \\[`Foo`]",
            "/// [`Foo` and `Bar`]",
            "/// [`Foo",
        ] {
            assert_eq!(resolve_all(line, false), line);
        }
    }

    #[test]
    fn parses_manifest() {
        let manifest = r#"
[package]
name = "ohos-foo-sys"
version = "0.1.0"

[dependencies]
# comment = "x"
arkui-sys = { version = "0.3", optional = true }
ohos-sys-opaque-types = { workspace = true }

[features]
name = []
"#;
        assert_eq!(
            parse_manifest(manifest),
            Some((
                "ohos_foo_sys".to_string(),
                vec!["arkui_sys".to_string(), "ohos_sys_opaque_types".to_string()]
            ))
        );
        assert_eq!(parse_manifest("[workspace]\nmembers = []"), None);
    }

    #[test]
    fn resolves_named_reexports() {
        let ast = syn::parse_file(
            "pub use err::Udmf_ErrCode; pub use other::{A, B as C}; pub use glob::*; use private::D;",
        )
        .unwrap();
        let reexports: Vec<_> = named_reexports(&ast)
            .into_iter()
            .map(|r| format!("{}::{} as {}", r.module, r.name, r.alias))
            .collect();
        assert_eq!(
            reexports,
            [
                "err::Udmf_ErrCode as Udmf_ErrCode",
                "other::A as A",
                "other::B as C"
            ]
        );

        let mut index = Index::default();
        let err = syn::parse_file(
            "pub struct Udmf_ErrCode(pub u32); \
             impl Udmf_ErrCode { pub const E_OK: Udmf_ErrCode = Udmf_ErrCode(0); } \
             pub struct Udmf_ErrCodeOther;",
        )
        .unwrap();
        index.add_items(0, &["err".to_string()], false, &err.items);
        for reexport in named_reexports(&ast) {
            index.add_reexport(0, &[], &reexport);
        }
        let mut c_names = CNames::default();
        c_names
            .enumerators
            .entry("UDMF_E_OK".to_string())
            .or_default()
            .push(Enumerator {
                c_enum: "Udmf_ErrCode".to_string(),
                rust_name: "E_OK".to_string(),
                is_zero: true,
            });
        let crates = crates();
        let resolver = Resolver {
            crates: &crates,
            index: &index,
            c_names: &c_names,
        };
        let resolve = |name| resolver.resolve(name, 0, &[]);
        assert_eq!(
            resolve("Udmf_ErrCode").as_deref(),
            Some("crate::Udmf_ErrCode")
        );
        assert_eq!(
            resolve("UDMF_E_OK").as_deref(),
            Some("crate::Udmf_ErrCode::E_OK")
        );
        assert_eq!(resolve("Udmf_ErrCodeOther"), None);
    }

    #[test]
    fn resolves_crate_glob_reexports_of_private_modules() {
        let ast = syn::parse_file(
            "pub use crate::a::*; pub use crate::m::{public::*, private::*}; use crate::x::*;",
        )
        .unwrap();
        assert_eq!(
            crate_glob_reexports(&ast),
            [vec!["a"], vec!["m", "public"], vec!["m", "private"]]
        );

        let mut index = Index::default();
        add_items(&mut index, 0, "m::public", "pub struct Public;");
        let private = syn::parse_file("pub struct Private(pub u32);").unwrap();
        let private_path = ["m".to_string(), "private".to_string()];
        index.add_items(0, &private_path, false, &private.items);
        for source in crate_glob_reexports(&ast) {
            index.add_glob_reexport(0, &["types".to_string()], &source);
        }
        let crates = crates();
        let resolver = Resolver {
            crates: &crates,
            index: &index,
            c_names: &CNames::default(),
        };
        let resolve = |name| resolver.resolve(name, 0, &["other".to_string()]);
        assert_eq!(
            resolve("Public").as_deref(),
            Some("crate::m::public::Public")
        );
        assert_eq!(resolve("Private").as_deref(), Some("crate::types::Private"));
    }

    fn add_items(index: &mut Index, krate: usize, module: &str, source: &str) {
        let module: Vec<_> = module.split("::").map(str::to_string).collect();
        index.add_items(
            krate,
            &module,
            true,
            &syn::parse_file(source).unwrap().items,
        );
    }

    fn crates() -> Vec<Crate> {
        ["a_sys", "b_sys", "c_sys"]
            .into_iter()
            .map(|name| Crate {
                name: name.to_string(),
                dependencies: if name == "b_sys" {
                    vec!["a_sys".to_string()]
                } else {
                    vec![]
                },
                lib_rs: PathBuf::new(),
            })
            .collect()
    }

    #[test]
    fn resolves_c_names() {
        let mut index = Index::default();
        add_items(
            &mut index,
            0,
            "types",
            r#"
            pub type ArkUiResult = Result<(), ArkUiErrorCode>;
            impl ArkUiErrorCode {
                pub const PARAM_INVALID: ArkUiErrorCode = ArkUiErrorCode(401);
            }
            pub struct Plain(pub u32);
            impl Plain {
                pub const PLAIN_ONE: Plain = Plain(1);
            }
            pub struct Point { pub x: i32 }
            pub mod Module { pub const VALUE: Type = 1; }
            extern "C" {
                pub fn OH_Foo();
            }
            "#,
        );
        add_items(&mut index, 1, "other", "extern \"C\" { pub fn OH_Foo(); }");
        let mut c_names = CNames::default();
        c_names
            .type_renames
            .insert("ArkUI_ErrorCode".to_string(), "ArkUiResult".to_string());
        for (c_name, c_enum, rust_name, is_zero) in [
            (
                "ARKUI_ERROR_CODE_NO_ERROR",
                "ArkUI_ErrorCode",
                "NO_ERROR",
                true,
            ),
            (
                "ARKUI_ERROR_CODE_PARAM_INVALID",
                "ArkUI_ErrorCode",
                "PARAM_INVALID",
                false,
            ),
            ("PLAIN_ONE", "Plain", "PLAIN_ONE", false),
            ("VALUE", "Module", "VALUE", false),
        ] {
            c_names
                .enumerators
                .entry(c_name.to_string())
                .or_default()
                .push(Enumerator {
                    c_enum: c_enum.to_string(),
                    rust_name: rust_name.to_string(),
                    is_zero,
                });
        }
        let crates = crates();
        let resolver = Resolver {
            crates: &crates,
            index: &index,
            c_names: &c_names,
        };
        let from_a = |name| resolver.resolve(name, 0, &["other".to_string()]);
        assert_eq!(
            from_a("ArkUI_ErrorCode").as_deref(),
            Some("crate::types::ArkUiResult")
        );
        assert_eq!(
            from_a("ARKUI_ERROR_CODE_NO_ERROR").as_deref(),
            Some("crate::types::ArkUiResult")
        );
        assert_eq!(
            from_a("ARKUI_ERROR_CODE_PARAM_INVALID").as_deref(),
            Some("crate::types::ArkUiErrorCode::PARAM_INVALID")
        );
        assert_eq!(
            from_a("ArkUI_ErrorCode#ARKUI_ERROR_CODE_PARAM_INVALID").as_deref(),
            Some("crate::types::ArkUiErrorCode::PARAM_INVALID")
        );
        assert_eq!(
            from_a("PLAIN_ONE").as_deref(),
            Some("crate::types::Plain::PLAIN_ONE")
        );
        assert_eq!(from_a("Point#x").as_deref(), Some("crate::types::Point::x"));
        assert_eq!(
            from_a("VALUE").as_deref(),
            Some("crate::types::Module::VALUE")
        );
        assert_eq!(from_a("OH_Foo").as_deref(), Some("crate::types::OH_Foo"));
        assert_eq!(from_a("Missing"), None);
        // Crate `b_sys` has its own `OH_Foo` and depends on `a_sys`.
        assert_eq!(
            resolver.resolve("OH_Foo", 1, &["x".to_string()]).as_deref(),
            Some("crate::other::OH_Foo")
        );
        assert_eq!(
            resolver
                .resolve("PLAIN_ONE", 1, &["x".to_string()])
                .as_deref(),
            Some("a_sys::types::Plain::PLAIN_ONE")
        );
        // Crate `c_sys` doesn't depend on `a_sys`.
        assert_eq!(resolver.resolve("PLAIN_ONE", 2, &["x".to_string()]), None);
    }
}
