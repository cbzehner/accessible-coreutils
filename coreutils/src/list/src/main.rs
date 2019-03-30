use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// List the contents of a file or directory
#[derive(StructOpt)]
struct Arguments {
    /// The path to list
    ///
    /// If no argument is provided, the current directory is listed
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Arguments::from_args();
    let path = args.path;
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            println!("{}", file_name);
        }
    }
}
