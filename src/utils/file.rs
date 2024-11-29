use std::fs;

pub fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    content.trim().to_string()
}
