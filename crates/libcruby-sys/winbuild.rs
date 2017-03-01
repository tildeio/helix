use std::env;

// FIXME: Only do this on Windows
fn main() {
  let root = env::var("HELIX_ROOT").unwrap_or(String::from("."));
  // FIXME: Don't hardcode Ruby version if possible
  println!("cargo:rustc-flags=-L {root}/def -l dylib=msvcrt-ruby230 -L {root}/ruby/windows_build -l dylib=helix_runtime", root=root);
}

