
## Install watchexec-cli for hot reload 

```
cargo install --locked watchexec-cli
```

## Setup Rust workspace packages

### Create application package

```
cargo new [crate name]
```

### Create library package

```
cargo new [crate name] --lib
```

## Add dependency

```
cargo add [crate name] -p [crate name]
```

## Run application for development

```
watchexec --restart --watch [crate name] cargo run -p [crate name]
```

## Test application

```
watchexec --restart --watch [crate name] cargo test -p [crate name]
```

## Format codebase with rustfmt

```
cargo fmt -p [crate name]
```

## Lint codebase with clippy

```
cargo clippy -p [crate name]
```
