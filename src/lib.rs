#![crate_name="deqnap"]
extern crate regex;

use std::io;
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn walk_and_remove_extraneous_files(dir: &Path, count: &mut u64) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(walk_and_remove_extraneous_files(&entry.path(), count));
            } else {
                let filename = match entry.file_name().into_string() {
                    Err(_) => continue,
                    Ok(filename) => filename
                };
                // Match extraneous files
                if match_extraneous_file(&filename) {
                    let path = entry.path();
                    println!("Removing file: {}", path.display());
                    try!(fs::remove_file(path));
                    *count += 1;
                }
            }
        }
    }
    Ok(())
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
