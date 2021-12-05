use std::fs;

///
/// Read the contents of a file as lines
///
pub fn read_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();

    contents.lines().map(str::to_string).collect()
}
