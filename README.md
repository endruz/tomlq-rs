# tomlq-rs

一个命令行 TOML 处理工具。

使用方法：

```bash
$ ./tomlq -V
tomlq 0.1.0
$ ./tomlq -h
tomlq 0.1.0
endruz <endruz@foxmail.com>
A command-line TOML processing tool.

USAGE:
    tomlq <KEY> <FILE>

ARGS:
    <KEY>     Key to query from the TOML file
    <FILE>    A TOML file to load

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
$ ./tomlq package.name Cargo.toml
tomlq
$ ./tomlq package.toml.version Cargo.toml
error: Key package.toml.version not found!
```
