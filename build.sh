#! /bin/bash
dfx build
rm ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm
dfx canister install ProxyManager_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm.gz -y

