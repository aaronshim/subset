extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

mod directory_files;
use directory_files::*;

mod file_comparable;
use file_comparable::*;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: subset [-q | -v] [-t] <dir1> <dir2>
       subset --help

subset lets you compare two directory structures.

We are going to check whether the files in dir1 are a subset of the files in dir2, regardless of directory structure.

We are going to check to see that every file under the directory structure in dir1 must be present somewhere in the dir2 directory structure, regardless of where in the directory structure or definitions of equality.

There are multiple definitions of file equality that you can specify using flags, but the default is a MD5 hash of the contents of the file. It is conceivable that you can define a custom equality strategy that relies on other parameters, such as file name, subdirectory location, metadata, EXIF data, etc. The possibilities are endless.

Common options:
    -h, --help         Show this usage message.
    -q, --quiet        Do not print all mappings.
    -v, --verbose      Print all mappings.
    -t, --trivial      Will swap out the MD5 comparison for a trivial comparison (everything is equal). (This is to test extensibility.)
";

// We should think about moving away from DocOpt soon since it uses RustcDecodable, whcih is deprecated in favor of serde?
/// Parsing comand line arguments here
#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dir1: String,
    arg_dir2: String,
    flag_quiet: bool,
    flag_verbose: bool,
    flag_trivial: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    println!("Comparing {} with {}", args.arg_dir1, args.arg_dir2);

    // Make sure both of our inputs are valid directories
    fs::read_dir(&args.arg_dir1).expect("Directory cannot be read!");
    fs::read_dir(&args.arg_dir2).expect("Directory cannot be read!");

    // Main logic
    // (We would ideally DRY-out the line that calls the compare function, but because then the let binding would need to hold FileComparable's of different types, it's hard to do.)
    match args.flag_trivial {
        true => {
            let mut comparator = file_comparable::TrivialComparator::new();
            compare(&mut comparator, &args.arg_dir1, &args.arg_dir2);
        }
        _ => {
            let mut comparator = file_comparable::Md5Comparator::new();
            compare(&mut comparator, &args.arg_dir1, &args.arg_dir2);
        }
    }
}

// We are extracting the main logic to this function, where the generic types will not interfere
fn compare<K, T>(comparator: &mut T, dir1: &String, dir2: &String) where K: Ord, T : FileComparable<K> {
    // We are going to construct a map of comparable -> file for every file in our superset directory
    let mut superset = BTreeMap::new();

    let superset_dirpath = PathBuf::from(&dir2);
    let mut superset_iter = DirectoryFiles::new(&superset_dirpath);

    for path in superset_iter.by_ref() {
        let path = path.path();
        match comparator.get_key(&path) {
            Some(hashed) => { superset.insert(hashed, path); },
            None => {}
        };
    }
    println!("{}", superset_iter);

    // And we are going to check it against every file in the subset directory
    let mut num_missing = 0;
    let subset_dirpath = PathBuf::from(&dir1);
    let mut subset_iter = DirectoryFiles::new(&subset_dirpath); // mut needed for .by_ref
    for path in subset_iter.by_ref() {
        let path = path.path();
        match comparator.get_key(&path) {
            Some(hashed) => {
                match superset.get(&hashed) {
                    Some(_) => {},
                    None => {
                        num_missing+=1;
                        println!("Could not find {} in {}", path.display(), superset_dirpath.display());
                    }
                }
            },
            None => {}
        };
    }
    println!("{}", subset_iter);

    // Final state
    println!("We are missing {} files in {}", num_missing, superset_dirpath.display());
}