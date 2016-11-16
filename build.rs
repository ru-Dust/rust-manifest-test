use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("windres")
        .args(&["--input", "src/manifest-test.rc", "--output-format=coff", "--output"])
        .arg(&format!("{}/manifest-test.res", out_dir))
        .status().unwrap();

    Command::new("ar")
        .args(&["crs", "libmanifest.a", "manifest-test.res"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=manifest");
}
