extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::fs;
use std::path::PathBuf;

mod directory_files;
use directory_files::*;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: subset [-q | -v] <dir1> <dir2>
       subset --help

subset lets you compare two directory structures.

Common options:
    -h, --help         Show this usage message.
    -q, --quiet        Do not print all mappings.
    -v, --verbose      Print all mappings.
";

// We should think about moving away from DocOpt soon since it uses RustcDecodable, whcih is deprecated in favor of serde?
/// Parsing comand line arguments here
#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dir1: String,
    arg_dir2: String,
    flag_quiet: bool,
    flag_verbose: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    println!("Comparing {} with {}", args.arg_dir1, args.arg_dir2);

    // Make sure both of our inputs are valid directories
    fs::read_dir(&args.arg_dir1).expect("Directory cannot be read!");
    fs::read_dir(&args.arg_dir2).expect("Directory cannot be read!");

    // just to make sure that we can display the results of the directory read
    let dirpath1 = PathBuf::from(&args.arg_dir1);
    let mut iter1 = DirectoryFiles::new(&dirpath1); // mut needed for .by_ref
    for path in iter1.by_ref() { // retain ownership so we can print final state
        println!("In first directory: {}", path.path().display());
    }
    println!("End state: {}", iter1);

    let dirpath2 = PathBuf::from(&args.arg_dir2);
    let mut iter2 = DirectoryFiles::new(&dirpath2);
    for path in iter2.by_ref() {
        println!("In second directory: {}", path.path().display());
    }
    println!("End state: {}", iter2);
}