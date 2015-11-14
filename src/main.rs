use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

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
    let p = Path::new(".");
    let _ = walk_dirs(p, &do_something);
}
