extern crate tempfile;

use std::fs;

fn main() {
    let mut files = fs::read_dir(".")
        .expect("Could not list directory contents");
    let file_one = files.next()
        .expect("Not sure if there are enough files")
        .expect("First file broke");
    let path_one = file_one.path();
    let file_two = files.last().expect("Not sure if there are enough files")
        .expect("Second file broke");
    let path_two = file_two.path();
    let pivot_dir = tempfile::tempdir_in(".")
        .expect("Could not create temporary directory");
    let pivot_path = pivot_dir.path()
        .join(path_one.file_name()
              .expect("First file is not good enough"));
    fs::rename(&path_one, &pivot_path)
        .expect("First move didn't succeed");
    fs::rename(&path_two, path_one)
        .expect("Second move didn't succeed");
    fs::rename(pivot_path, path_two)
        .expect("Third move didn't succeed");
}
