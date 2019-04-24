# Lox interpreter

Currently only lexes syntax into tokens

## Usage

```shell
cargo run filename.lox
```

or from stdin

```shell
cat | cargo run -- - <<EOF
var foobar = "asdf"
EOF
```