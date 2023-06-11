#! /bin/bash
dfx build

## install with cargo install ic-wasm
ic-wasm -o ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend_optimized.wasm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm shrink
rm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend_optimized.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend_optimized.wasm
dfx canister install Xtreamly_Canister_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend_optimized.wasm.gz -y

