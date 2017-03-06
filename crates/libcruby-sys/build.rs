use std::{env,fs};
use std::path::Path;
use std::process::Command;

fn main() {
  let target = env::var("TARGET").unwrap();
  if target.contains("windows") {
    let root_str = env::var("HELIX_ROOT").unwrap_or(String::from("."));
    let out_dir_str = env::var("OUT_DIR").expect("OUT_DIR required");
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION required");

    let root = Path::new(root_str.as_str());
    let out_dir = Path::new(out_dir_str.as_str());

    // TODO: Clean this all up. There has to be a prettier way.

    // Read info about current Ruby install
    let raw_ruby_info = Command::new("ruby")
                                .arg(root.join("scripts").join("ruby_info.rb"))
                                .output()
                                .expect("failed to get Ruby info");
    let raw_ruby_output = String::from_utf8_lossy(&raw_ruby_info.stdout);
    let mut raw_ruby_lines = raw_ruby_output.lines();
    let ruby_libdir = Path::new(raw_ruby_lines.next().expect("Ruby info has no libdir"));
    let libruby = raw_ruby_lines.next().expect("Ruby info has no LIBRUBY");
    let libruby_so = raw_ruby_lines.next().expect("Ruby info has no LIBRUBY_SO");
    if raw_ruby_lines.next() != None {
      panic!("Unexpected information returned in Ruby info");
    }

    let ruby_libname = libruby_so.split('.').next().expect("can't extract Ruby lib name");

    // Copy .dll.a file to .lib since Rust msvc looks for .lib files only
    fs::copy(ruby_libdir.join(libruby), out_dir.join(ruby_libname).with_extension("lib"))
        .expect("unable to copy libruby");

    let windows_build_dir = root.join("ruby").join("windows_build");

    // Set up linker
    println!("cargo:rustc-flags=-L {libpath} -l dylib={libruby} -L {windows_build} -l helix-runtime:helix-runtime-{version}",
              libpath=out_dir.to_str().expect("can't get str from out_dir"),
              libruby=ruby_libname,
              windows_build=windows_build_dir.to_str().expect("can't get str from root"),
              version=version.replace(".", "-"));
  }
}

