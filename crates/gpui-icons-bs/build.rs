use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set by cargo");
    let icons_dir = Path::new(&manifest_dir).join("assets/icons");

    if !icons_dir.is_dir() {
        panic!(
            "expected default icons at {}, but the directory is missing",
            icons_dir.display(),
        );
    }

    println!("cargo:icons-dir={}", icons_dir.display());
    println!("cargo:rerun-if-changed=assets/icons");
    println!("cargo:rerun-if-changed=build.rs");
}
