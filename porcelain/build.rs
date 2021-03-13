use std::env;
use std::fs;
use std::path::Path;

// Read accessible-coreutils.yaml
// Iterate through the top-level keys and create a binary for each one

fn main() {
    let executable = "list";
    // Intentionally bypass the recommendation to rely on OUT_DIR
    let out_dir = format!("{}/src/bin/", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("Output directory is {}", out_dir);
    println!("Executable is {}", executable);
    let dest_path = Path::new(&out_dir).join(format!("{}.rs", executable));
    fs::write(
        &dest_path,
        format!(
            r###"
#[macro_use]
extern crate clap;

use porcelain::build_executable;

fn main() {{
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("../../configs/english/{}.yaml");
    build_executable(yaml);
}}"###,
            executable
        ),
    )
    .unwrap();
    // println!("cargo:rerun-if-changed=build.rs");
}
