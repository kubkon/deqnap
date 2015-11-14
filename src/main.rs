extern crate rustc_serialize;
extern crate docopt;
extern crate deqnap;

use docopt::Docopt;
use std::path::Path;
use deqnap::walk_and_remove_extraneous_files;

const USAGE: &'static str = "
DEQNAPify your Dropbox.

Usage:
    deqnap <path>
    deqnap (-h | --help)
    deqnap --version

Options:
    -h --help   Prints this message.
    --version   Prints version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: String,
}

fn main() {
    // Parse arguments
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let path = Path::new(&args.arg_path);
    match walk_and_remove_extraneous_files(path) {
        Err(err) => panic!("Unexpected error occurred: {}", err),
        Ok(()) => println!("Finished removing all extraneous files.")
    }
}
