cd ext
ruby extconf.rb
make
cd ..
cargo build --release
gcc -Wl,-force_load,target/release/libconsole.a --shared -Wl,-undefined,dynamic_lookup -o libconsole.bundle
