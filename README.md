_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate rksm/rust-hot-reload`._

# {{project-name}}

A workflow for quick feedback while writing Rust! See https://robert.kra.hn/posts/hot-reloading-rust.html for more.

## Usage

### Hot reload for development

For development use two terminals and run the binary

```shell
cargo watch -w 'crates/bin' -x 'run --features reload'
```

and (re-)build the lib

```shell
cargo watch -w crates/lib -x build
```

### Statically build for release

```shell
cargo build --release
```
