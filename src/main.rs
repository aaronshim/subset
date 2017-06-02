extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

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
}