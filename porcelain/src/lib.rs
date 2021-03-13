use std::process;

#[macro_use]
extern crate clap;
use clap::{App, Arg};
use yaml_rust::Yaml;

// TODO: Generate completions for all shells (https://docs.rs/clap/2.33.3/clap/enum.Shell.html)

const FAILURE_UNKNOWN: i32 = -1;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub fn build_executable(yaml: &Yaml) {
    let matches = App::from_yaml(yaml)
        // Global arguments available in all binaries
        .args(&[
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
            Arg::with_name("version")
                .long("version")
                .help("Show the current version"),
        ])
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();

    // Globally present options
    // - version
    // - verbosity
    // - handle colorization based on whether it's part of a pipe or not
    // - ???
    if matches.is_present("version") {
        println!("{}", crate_version!());
        process::exit(0)
    }

    match command("ls", vec!["-l", "-a", "."]) {
        Ok(success) => process::exit(success),
        Err(message) => {
            eprintln!("Error: {}", message);
            process::exit(FAILURE_UNKNOWN)
        }
    }
}

fn command(binary_name: &str, arguments: Vec<&str>) -> Result<i32, String> {
    let child = process::Command::new(binary_name)
        .args(&arguments)
        .spawn()
        .map_err(|error| error.to_string())?;
    let output = child
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    return output.status.code().ok_or("Missing ExitStatus".into());
}
