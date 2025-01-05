# ohos-sys-opaque-types

This is an internal helper library for the `ohos-sys` crates, which defines opaque types,
which are used across multiple components of the ohos-sys crates.
In C/C++ these types are forward declared in multiple different headers, but doing the same
in Rust would lead to multiple incompatible types. 
To avoid such needless pointer casts, we use this library as a common source for the opaque types,
to get interoperability across the different ohos-sys crates.

## License

These bindings are licensed under the [Apache 2.0](./LICENSE) license, matching
the license of corresponding C/C++ header files.
