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
    let mut count_removed = 0u64;
    match walk_and_remove_extraneous_files(path, &mut count_removed) {
        Err(err) => panic!("Unexpected error occurred: {}", err),
        Ok(()) => {
            if count_removed > 0 {
                println!("Finished. Removed {} extraneous files.", count_removed);
            } else {
                println!("No extraneous files found.");
            }
        }
    }
}
