
cargo install cross

cross build --bin nsv --release --target i686-pc-windows-gnu &
cross build --bin nsv --release --target x86_64-pc-windows-gnu

wait
