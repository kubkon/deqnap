extern crate rustc_serialize;
extern crate docopt;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use docopt::Docopt;

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

fn walk_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(walk_dirs(&entry.path(), cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn do_something(entry: &DirEntry) {
    println!("{}", entry.file_name().to_str().unwrap());
}

fn main() {
    // Parse arguments
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let path = Path::new(&args.arg_path);
    match walk_dirs(path, &do_something) {
        Err(err) => panic!("Unexpected error occurred: {}", err),
        Ok(()) => ()
    }
}
