cargo install cross

wait

cross build --bin nsv --release --verbose --target aarch64-unknown-linux-gnu
cross build --bin nsv --release --verbose --target i686-unknown-linux-gnu
cargo build --bin nsv --release --verbose --target x86_64-unknown-linux-gnu
