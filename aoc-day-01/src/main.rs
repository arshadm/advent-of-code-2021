fn main() {
    println!("Hello, world!");
}

fn depth_increases(measurements: &Vec<i32>) -> usize {
    measurements.iter().zip(&measurements[1..])
        .filter(|pair| pair.0 < pair.1)
        .count()
}

fn depth_increases_sliding(measurements: &Vec<i32>) -> usize {
    let data: Vec<i32> = measurements.iter().zip(&measurements[1..]).zip(&measurements[2..])
        .map(|triple| triple.0.0 + triple.0.1 + triple.1)
        .collect();

    return depth_increases(&data);
}

#[cfg(test)]
mod tests {
    use crate::{depth_increases, depth_increases_sliding};

    #[test]  // Part 1
    fn test_depth_increases_part1() {
        let depths = read_depth_data();

        assert_eq!(depth_increases(&depths), 1688);
    }

    #[test]  // Part 2
    fn test_depth_increases_part2() {
        let depths = read_depth_data();

        assert_eq!(depth_increases_sliding(&depths), 1728);
    }

    fn read_depth_data() -> Vec<i32> {
        let lines: Vec<String> = util::read_lines("./src/data.txt");

        lines.iter()
            .filter_map(|str| str.parse::<i32>().ok())
            .collect()
    }
}
