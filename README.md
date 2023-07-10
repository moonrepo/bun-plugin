# Bun plugin

[Bun](https://bun.sh/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands. Requires proto >= v0.12.

```shell
proto install bun-test
proto list-remote bun-test
```
