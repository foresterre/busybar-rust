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
    cargo msrv verify -- cargo check -p busylib
    cargo msrv verify -- cargo check -p busybar

msrv-find:
    cargo msrv find -- cargo check -p busylib
    cargo msrv find -- cargo check -p busybar

precommit: fmt check clippy test
    just msrv-verify || { just msrv-find; exit 1; }
