use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // TODO:
    // - Read accessible-coreutils.yaml
    // - Iterate through the top-level keys and create a binary for each one
    // - Support multiple languages
    let executable = "list";

    // Intentionally bypass the recommendation to rely on OUT_DIR
    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path =
        Path::new(&format!("{}/src/bin/", project_root)).join(format!("{}.rs", executable));
    fs::write(
        &dest_path,
        format!(
            r###"
#[macro_use]
extern crate clap;

use porcelain::build_executable;

fn main() {{
    let yaml = load_yaml!("{}/configs/english/{}.yaml");
    build_executable(yaml);
}}"###,
            project_root, executable
        ),
    )
    .unwrap();
    // println!("cargo:rerun-if-changed=build.rs");
}
