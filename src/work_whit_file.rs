use std::fs::{File, OpenOptions};
use std::io::{Write , Read};
use std::fs;
use std::path::Path;



pub fn file_exists(file_path: String) -> bool {
    Path::new(file_path.as_str()).exists()

}

pub fn create_file(path: String) {
    File::create(path).expect("creation failed");

}
pub fn remove_file(path: String) {  // not tested
    fs::remove_file(path).expect("could not remove file");

}

pub fn read_file(path: String) -> String {
    if ! file_exists(path.to_string()) {
        let path_clone = path.clone();
        create_file(path);
        return "".to_string();
    }

    let mut data_file = File::open(path).unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    file_content

}

pub fn append_file(path: String, content: String) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(content.as_bytes())
        .expect("write failed");

}

pub fn write_file(path: String, content: String) {
    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // This ensures the file is overwritten
        .open(path)
        .expect("cannot open file");

    data_file
        .write_all(content.as_bytes())
        .expect("write failed");
}