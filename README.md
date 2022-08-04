_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate rksm/rust-hot-reload`._

# {{project-name}}

A workflow for quick feedback while writing Rust! See https://robert.kra.hn/posts/hot-reloading-rust/ and https://crates.io/crates/hot-lib-reloader for more info.

## Usage

### Hot reload for development

For development use two terminals and run the binary

```shell
$ cargo watch -i lib -x 'run --features reload'
```

and (re-)build the lib

```shell
$ cargo watch -w lib -x 'build -p lib'
```

### Statically build or run for release

```shell
cargo build --release
cargo run --release
```
