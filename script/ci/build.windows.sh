
cargo install cross

cross build --bin nsv --release --target i686-pc-windows-msvc &
cross build --bin nsv --release --target x86_64-pc-windows-msvc &
cross build --bin install --release --target i686-pc-windows-msvc &
cross build --bin install --release --target x86_64-pc-windows-msvc

wait

mv target/i686-pc-windows-msvc/release/nsv.exe target/nsv-x86-win.exe
mv target/x86_64-pc-windows-msvc/release/nsv.exe target/nsv-x64-win.exe

mv target/i686-pc-windows-msvc/release/install.exe target/install-x86-win.exe
mv target/x86_64-pc-windows-msvc/release/install.exe target/install-x64-win.exe
