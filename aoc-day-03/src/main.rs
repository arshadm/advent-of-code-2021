use std::borrow::Borrow;
use std::fs::read;
use std::i32;

fn main() {
    println!("Hello, world!");
}

fn compute_gamma_epsilon_part1(readings: &Vec<String>, num_bits: usize) -> (i32, i32) {
    let components = split_components(readings);
    let nentries = components.len();

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for i in 0..num_bits {
        let count = get_count(components, '1', i);
        let ch = if count > nentries / 2 { ('1', '0') } else { ('0', '1') };
        gamma.push(ch.0);
        epsilon.push(ch.1);
    }

    (
        i32::from_str_radix(gamma.as_str(), 2).unwrap(),
        i32::from_str_radix(epsilon.as_str(), 2).unwrap()
    )
}

fn compute_oxygen_rating(readings: Vec<Vec<char>>, num_bits: usize, bit_pos: usize) -> usize {
    let num_readings = readings.len();
    let count = get_count(readings, '1', bit_pos);
    let ch = if (count >= num_readings / 2) { '1' } else { '0' };
    let filtered_readings: Vec<&Vec<char>> = readings.iter()
        .filter(|row| row[bit_pos] == ch)
        .collect();

    if (filtered_readings.len() == 1) {
        let reading = filtered_readings.get(0).unwrap();
        i32::from_str_radix(reading.join("").as_str(), 2).unwrap()
    } else {
        compute_oxygen_rating(&filtered_readings, num_bits, bit_pos + 1)
    }
}

//
// Split a reading into individual components (e.. 0101 -> [0, 1, 0, 1]
//
fn split_components(readings: &Vec<String>) -> Vec<Vec<char>> {
    readings.iter()
        .filter(|str| !str.is_empty())
        .map(|str| str.chars().collect())
        .collect()
}

//
// Get the count of entries at a certain position
//
fn get_count(bits: Vec<Vec<char>>, ch: char, idx: usize) -> usize {
    bits.iter()
        .map(|row| row.get(idx).unwrap())
        .filter(|c| **c == ch)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{compute_gamma_epsilon_part1, compute_oxygen_rating, split_components};

    #[test]  // Part 1
    fn test_gamme_epsilon_part1() {
        let readings = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let (gamma, epsilon) = compute_gamma_epsilon_part1(&readings, 5);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]  // Part 1 - With data
    fn test_gamma_epsilon_part1_with_data() {
        let readings = util::read_lines("./src/data.txt");

        let (gamma, epsilon) = compute_gamma_epsilon_part1(&readings, 12);
        assert_eq!(gamma * epsilon, 3885894);
    }

    #[test] // Part 2 - basic data
    fn test_gamma_epsilon_part2() {
        let readings = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let components = split_components(&readings);

        let oxygen = compute_oxygen_rating(components, 5, 0);
        print!("Oxygen: {}", oxygen);
    }
}
