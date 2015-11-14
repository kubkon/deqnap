#![crate_name="deqnap"]
extern crate regex;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use regex::Regex;

pub fn walk_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
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

pub fn do_something(entry: &DirEntry) {
    println!("{}", entry.file_name().to_str().unwrap());
}

pub fn match_extraneous_file(name: &str) -> bool {
    let re = Regex::new(r"_\d{8}T\d{6}\.\d{6}Z$").unwrap();
    re.is_match(name)
}

#[test]
fn test_match_extraneous_file() {
    // Should match
    assert!(match_extraneous_file("WP_20150727_14_34_31_Pro.mp4_20150916T145333.259861Z"));
    assert!(match_extraneous_file("1.1, edited, IMG_1485.jpg_20150912T154653.851434Z"));
    assert!(match_extraneous_file("STM, wand+tablet.jpg_20150912T154029.711724Z"));
    assert!(match_extraneous_file("IMG_1485.JPG_20150912T153657.558194Z"));

    // Shouldn't match
    assert!(!match_extraneous_file("WP_20150727_14_34_31_Pro.mp4"));
    assert!(!match_extraneous_file("1.1, edited, IMG_1485.jpg"));
    assert!(!match_extraneous_file("STM, wand+tablet.jpg"));
    assert!(!match_extraneous_file("IMG_1485.JPG"));
}
