fn main() {
  //println!("cargo:include=c:/Users/Peter/Development/helix");
  println!("cargo:rustc-flags=-L c:/Users/Peter/Development/helix -l dylib=msvcrt-ruby230 -l dylib=helix_runtime");
}

