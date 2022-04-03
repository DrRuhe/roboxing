
monitor:
    watchexec -r -e rs just play

play:
    cargo run --bin roboxing-bin

prepare:
    cargo check
    cargo doc


test package :
    cargo test --package {{package}} --doc
    cargo clippy --package {{package}} -- --no-deps
    cargo test --package {{package}}

wasm:
    cargo run --target wasm32-unknown-unknown --bin roboxing-bin

format:
    cargo fmt
    cargo clippy --fix --no-deps