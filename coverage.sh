cargo clean
export RUSTFLAGS="-Z instrument-coverage"
export LLVM_PROFILE_FILE="coverage-%m.profraw"
cargo test --tests
