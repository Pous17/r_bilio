# Rbiblio
This project was a school project where we were asked to provide a system to manage a database. In this case, a database that would be used by a Library. When most of my classmates did web interfaces, i chose to create a CLI in Rust, for efficiency reasons, as we wanted our system to be scalable since a Library can have thousands of entries. 

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
