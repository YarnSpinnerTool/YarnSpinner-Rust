Terminal examples that run Yarn dialogue with the `yarnspinner` crate directly.

The dialogue has to be compiled with
[`ysc`](https://github.com/YarnSpinnerTool/YarnSpinner-Console) first. From this
directory:

```sh
for f in assets/dialogue/*.yarn; do ysc compile "$f" -o assets/dialogue/; done
cargo run --bin hello_world
```
