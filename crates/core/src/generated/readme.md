# Compiler API

Rust code is generated via [`prost_build`](https://github.com/tokio-rs/prost/tree/master/prost-build) in the `generate_proto` binary of `yarnspinner_codegen`.
Running this requires pulling in the submodules:

```bash
git submodule update --init --recursive
```

As well as installing `protoc`
