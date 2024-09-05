cargo install cross

wait

cross build --bin nsv --release --verbose --target aarch64-unknown-linux-gnu &
cross build --bin nsv --release --verbose --target i686-unknown-linux-gnu &
cargo build --bin nsv --release --verbose --target x86_64-unknown-linux-gnu

wait

mv target/x86_64-unknown-linux-gnu/release/nsv target/nsv-x64-linux
mv target/i686-unknown-linux-gnu/release/nsv target/nsv-x86-linux
mv target/aarch64-unknown-linux-gnu/release/nsv target/nsv-arm64-linux
