default:
    @just --list

check:
    cargo check --workspace

test:
    cargo test --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace

msrv-verify:
    cargo msrv verify --manifest-path crates/busylib/Cargo.toml
    cargo msrv verify --manifest-path crates/busybar/Cargo.toml

msrv-find:
    cargo msrv find --min 1.85 --manifest-path crates/busylib/Cargo.toml --write-msrv -- cargo check -p busylib
    cargo msrv find --min 1.85 --manifest-path crates/busybar/Cargo.toml --write-msrv -- cargo check -p busybar

precommit: fmt check clippy test
    just msrv-verify || { just msrv-find; exit 1; }

book:
    mdbook serve --open book

proto-update *dir:
    cargo run -p busylib-proto-build -- {{dir}}
    cargo fmt --all
