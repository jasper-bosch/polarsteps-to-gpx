default:
    just --list

build:
    cargo build --release

ci:
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo machete
    cargo outdated -R

clean:
    cargo clean

debug:
    cargo build

docs:
    cargo doc --open

fix:
    cargo fix --allow-dirty

install:
    cargo install --path .

lint:
    cargo clippy -- -D warnings

rebuild: clean build

run:
    cargo run

test:
    cargo test
