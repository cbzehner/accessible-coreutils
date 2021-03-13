use std::process;

#[macro_use]
extern crate clap;
use clap::App;

// TODO: Generate completions for all shells (https://docs.rs/clap/2.33.3/clap/enum.Shell.html)

const FAILURE_UNKNOWN: i32 = -1;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("list.yaml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();

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
    // process::Command::new(binary_name).args(&arguments).spawn().map_err(op)

    let child = process::Command::new(binary_name)
        .args(&arguments)
        .spawn()
        .map_err(|error| error.to_string())?;
    let output = child
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    return output.status.code().ok_or("Missing ExitStatus".into());

    // match process::Command::new(binary_name).args(&arguments).spawn() {
    //     Ok(child) => match child.wait_with_output() {
    //         Ok(output) => match output.status.code() {
    //             Some(code) => process::exit(code),
    //             None => process::exit(FAILURE_UNKNOWN),
    //         },
    //         // TODO: Send error message to stdout and exit with failure unknown
    //         Err(error) => eprintln!("{}", error.to_string()),
    //     },
    //     // TODO: Send error message to stdout and exit with failure unknown
    //     Err(error) => eprintln!("{}", error.to_string()),
    // }
}

// fn exit_on_error(error: std::error::Error) {
//     eprintln!("{}", error.to_string());
//     process::exit(FAILURE_UNKNOWN);
// }
