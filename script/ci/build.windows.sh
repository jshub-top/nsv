
cargo install cross

cross build --bin nsv --release --target i686-pc-windows-gnu &
cross build --bin nsv --release --target x86_64-pc-windows-gnu

wait

mv target/i686-pc-windows-gnu/release/nsv.exe target/nsv-x86-win.exe
mv target/x86_64-pc-windows-gnu/release/nsv.exe target/nsv-x64-win.exe
