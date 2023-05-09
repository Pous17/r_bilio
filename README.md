# Rbiblio

## Install
### Rust
https://www.rust-lang.org/tools/install

### Cargo
https://doc.rust-lang.org/cargo/getting-started/installation.html

### Diesel
```
cargo install diesel_cli
```

## Setup
```
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```
```
diesel setup
```
```
diesel migration run
```

## Launch
```
cargo run
```

### Usefull commands

redo migration
```
diesel migration redo
```
