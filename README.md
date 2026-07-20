# tomlfmt

tomlfmt is a simple and fast toml prettifier written in rust


## Installation

```shell
cargo install tomlfmt
```


## Usage


### write-in-place (`-w`)

```shell
tomlfmt -w file1.toml file2.toml
```


### write to stdout

```shell
tomlfmt file.toml > prettified-file.toml
```
