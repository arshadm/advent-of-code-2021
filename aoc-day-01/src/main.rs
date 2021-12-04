fn main() {
    println!("Hello, world!");
}

fn depth_increases(measurements: &Vec<i32>) -> usize {
    measurements.iter().zip(&measurements[1..])
        .filter(|pair| pair.0 < pair.1)
        .count()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::depth_increases;

    #[test]
    fn test_depth_increases() {
        let contents = fs::read_to_string("./src/data.txt").unwrap();
        let lines: Vec<&str> = contents
            .split("\n")
            .collect();

        let depths = lines.iter()
            .filter(|str| str.len() > 0)
            .map(|str: &&str| str.parse::<i32>().unwrap())
            .collect();

        assert_eq!(depth_increases(&depths), 1688);
    }
}
