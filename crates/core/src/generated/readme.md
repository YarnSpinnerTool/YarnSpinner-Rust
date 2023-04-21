# Compiler API

Rust code is generated via [`prost_build`](https://github.com/tokio-rs/prost/tree/master/prost-build) in the `build.rs`.
This requires pulling in the submodules:

```bash
git submodule update --init --recursive
```
