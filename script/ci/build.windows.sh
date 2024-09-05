rustup target add aarch64-pc-windows-msvc &
rustup target add i686-pc-windows-msvc &
rustup target add x86_64-pc-windows-msvc

wait

cargo build --bin nsv --release --verbose --target aarch64-pc-windows-msvc
cargo build --bin nsv --release --verbose --target i686-pc-windows-msvc
cargo build --bin nsv --release --verbose --target x86_64-pc-windows-msvc
