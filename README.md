# Rbiblio

## Install
rustc
cargo
diesel

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
cargo run --bin front
```

### Usefull commands

redo migration
```
diesel migration redo
```
