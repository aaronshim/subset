#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use std::fs;
use std::path::PathBuf;

mod directory_files;
use directory_files::*;

mod file_comparable;

mod directory_comparable;
use directory_comparable::*;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: subset [-q | -v] [-t | -n] [-b] <dir1> <dir2>
       subset --help

subset lets you compare two directory structures.

We are going to check whether the files in dir1 are a subset of the files in dir2, regardless of directory structure.

We are going to check to see that every file under the directory structure in dir1 must be present somewhere in the dir2 directory structure, regardless of where in the directory structure or definitions of equality.

There are multiple definitions of file equality that you can specify using flags, but the default is a MD5 hash of the contents of the file. It is conceivable that you can define a custom equality strategy that relies on other parameters, such as file name, subdirectory location, metadata, EXIF data, etc. The possibilities are endless.

Common options:
    -h, --help           Show this usage message.
    -q, --quiet          Do not print all mappings.
    -v, --verbose        Print all mappings.
    -t, --trivial        Will swap out the MD5 comparison for a trivial comparison (everything is equal). (This is to test extensibility.)
    -n, --name           Will swap out the MD5 comparison for a filename comparison.
    -b, --bidirectional  Also check whether dir2 is also a subset of dir1 (essentially, set equality) and print out missing lists for both directories.
";

// We should think about moving away from DocOpt soon since it uses RustcDecodable,
//  which is deprecated in favor of serde?
/// Parsing comand line arguments here
#[derive(Debug, Deserialize)]
struct Args {
    arg_dir1: String,
    arg_dir2: String,
    flag_quiet: bool,
    flag_verbose: bool,
    flag_trivial: bool,
    flag_name: bool,
    flag_bidirectional: bool,
}

/// This should be the UI layer as much as possible-- it parses the command line arguments,
/// hands it off to our business logic, and then collects the answers back and print them.
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Comparing {} with {}", args.arg_dir1, args.arg_dir2);

    // Make sure both of our inputs are valid directories
    fs::read_dir(&args.arg_dir1).expect("Directory cannot be read!");
    fs::read_dir(&args.arg_dir2).expect("Directory cannot be read!");

    // Main logic: using dynamic dispatch
    // (I don't feel too bad about boxing here because this is essentially a singleton.)
    let mut program: Box<DirectoryComparable> = if args.flag_trivial {
        Box::new(TrivialDirectoryComparable {})
    } else if args.flag_name {
        let filename_comparator = file_comparable::FileNameComparable::new();
        Box::new(DirectoryComparableWithFileComparable::new(
            filename_comparator,
        ))
    } else {
        let md5_comparator = file_comparable::Md5Comparable::new();
        Box::new(DirectoryComparableWithFileComparable::new(md5_comparator))
    };

    let superset_dirpath = PathBuf::from(&args.arg_dir2);
    // eww... why do we have to coerce these Box types again?
    // (again, only two of these Box types in existence so not so bad...)
    let mut superset_iter: Box<Iterator<Item = PathBuf>> =
        Box::new(DirectoryFiles::new(&superset_dirpath));

    let subset_dirpath = PathBuf::from(&args.arg_dir1);
    let mut subset_iter: Box<Iterator<Item = PathBuf>> =
        Box::new(DirectoryFiles::new(&subset_dirpath)); // mut needed for .by_ref

    if args.flag_bidirectional {
        // Run program
        let (subset_missing_result, superset_missing_result) =
            program.report_missing_bidirectional(&mut subset_iter, &mut superset_iter);

        // View layer (printing)
        for missing_file in subset_missing_result.iter() {
            println!(
                "Could not find {} in {}",
                missing_file.display(),
                superset_dirpath.display()
            );
        }

        println!(
            "\nWe are missing {} files in {}\n",
            subset_missing_result.len(),
            superset_dirpath.display()
        );

        for missing_file in superset_missing_result.iter() {
            println!(
                "Could not find {} in {}",
                missing_file.display(),
                subset_dirpath.display()
            );
        }

        println!(
            "\nWe are missing {} files in {}",
            superset_missing_result.len(),
            subset_dirpath.display()
        );
    } else {
        // Run program
        let result = program.report_missing(&mut subset_iter, &mut superset_iter);

        // View layer (printing)
        for missing_file in result.iter() {
            println!(
                "Could not find {} in {}",
                missing_file.display(),
                superset_dirpath.display()
            );
        }

        println!(
            "\nWe are missing {} files in {}",
            result.len(),
            superset_dirpath.display()
        );
    }
}
