rustup target add x86_64-apple-darwin &
rustup target add aarch64-apple-darwin

wait

cargo build --bin nsv --release --target x86_64-apple-darwin &
cargo build --bin nsv --release --target aarch64-apple-darwin

wait
