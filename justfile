build_wallet:   
    cargo build -p wallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/wallet.wasm wallet.wasm

install_wallet:
    dfx canister install --mode reinstall --wasm wallet.wasm wallet --argument '("regtest")'