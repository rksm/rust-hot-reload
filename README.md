_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate rksm/rust-hot-reload`._

# {{project-name}}

A workflow for quick feedback while writing Rust! See https://github.com/rksm/hot-lib-reloader-rs and https://robert.kra.hn/posts/hot-reloading-rust/ for more info.

## Usage

### Hot reload for development

For development use two terminals to run the binary and (re-)build the lib:

```shell
$ cargo watch -i lib -x 'run --features reload'
$ cargo watch -w lib -x 'build -p lib'
```

With [cargo runcc](https://crates.io/crates/runcc) you just need to run `cargo runcc -c runcc.yml`.

### Statically build or run for release

```shell
cargo build --release
cargo run --release
```
