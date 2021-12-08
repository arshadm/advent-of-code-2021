use std::fs;

///
/// Read the contents of a file as lines
///
pub fn read_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();

    contents.lines().map(str::to_string).collect()
}

///
/// Parse a list of comma separated numbers into a vector
///
pub fn parse_numbers(str: &String) -> Vec::<i32> {
    str.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}
