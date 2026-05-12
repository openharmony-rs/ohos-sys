# Generator patches

Unified-diff patches applied by `scripts/generator` after bindgen and
`cargo +nightly fmt`. Use one when the bindgen output cannot be fixed by tweaking
the generator config — most commonly when an upstream C header has malformed
doxygen comments that bindgen attaches to the wrong member.

## Conventions

- Each `*.patch` is a unified diff with paths relative to the repo root and a
  `--- a/PATH` / `+++ b/PATH` header. Applied via `git apply` from the repo root.
- Patches are applied in alphabetical order.
- The patch's "before" side must match the freshly generated, formatted output
  exactly. If bindgen output drifts, the patch will fail loudly and must be
  regenerated.
- Name patches after the file/issue they fix, e.g.
  `native_buffer_usage_comments.patch`.

## Generating a new patch

```sh
REL=path/to/generated_ffi.rs
cp "$REL" /tmp/before.rs
# ... hand-edit "$REL" into the desired state ...
diff -u --label "a/$REL" --label "b/$REL" /tmp/before.rs "$REL" \
    > scripts/generator/patches/<descriptive-name>.patch
```

Confirm before committing:

```sh
git apply --check scripts/generator/patches/<descriptive-name>.patch
```
