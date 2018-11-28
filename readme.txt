# init
cargo init project_name
cargo init .

# building project
cargo build
cargo build --release

# testing
cargo test
cargo test test_filename

# compile and run (incremental)
cargo run

# install rust utils
cargo install crate_name
cargo install iron
# http://diesel.rs/guides/getting-started/
cargo install diesel


# install openssl-bin
# GOOD crates in https://crates.io/
cargo install cargo-edit

# GOOD linters:
cargo install clippy
cargo clippy

# publish to crates.io
cargo publish


# main.rs
# lib.rs


# DEPENDENCIES (from cargo-edit crate)
cargo add package_name
cargo add package_name=="2.0.1"
cargo rm package_name
# read features

# CRATES:
# rayon - parallel iterator
# treads_pool -
# mio - async i/o
# std::net
# reqwest
# hyper - для низкоуровневая либа для хттп
# serde - сериализование типов. разные крэйты для разных типов


ldd
