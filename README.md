# Bun plugin

[Bun](https://bun.sh/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
bun = "source:https://github.com/moonrepo/bun-plugin/releases/download/vX.Y.Z/bun_plugin.wasm"
```

## Configuration

Bun plugin does not support configuration.

## Hooks

Bun plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install bun-test
proto list-remote bun-test
```
