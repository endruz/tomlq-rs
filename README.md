# tomlq-rs

一个命令行 TOML 处理工具。

使用方法：

```bash
$ ./tomlq-rs -V
tomlq-rs 0.1.0
$ ./tomlq-rs -h
tomlq-rs 0.1.0
endruz <endruz@foxmail.com>
A command-line TOML processing tool.

USAGE:
    tomlq-rs <KEY> <FILE>

ARGS:
    <KEY>     Key to query from the TOML file
    <FILE>    A TOML file to load

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
$ ./tomlq-rs package.name Cargo.toml
tomlq-rs
$ ./tomlq-rs package.toml.version Cargo.toml
error: Key package.toml.version not found!
```
