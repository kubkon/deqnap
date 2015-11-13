use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


fn main() {
    let p = Path::new(".");
    let entries = match fs::read_dir(p) {
        Err(err) => panic!(err),
        Ok(e) => e
    };
    for entry in entries {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let meta = match fs::metadata(entry_path) {
            Err(err) => panic!(err),
            Ok(meta) => meta
        };
        if meta.is_file() {
            println!("{}", entry.file_name().to_str().unwrap());
        }
    }
}
