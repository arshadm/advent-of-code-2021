use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn final_position_part1(steps: &Vec<String>) -> (i32, i32) {
    let steps_re = Regex::new(r#"(forward|down|up) (\d+)"#).unwrap();

    steps.iter()
        .filter(|str| !str.is_empty())
        .map(|step| {
            let captures = steps_re.captures(step).unwrap();
            let direction = captures.get(1).unwrap().as_str();
            let amount = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

            match direction {
                "forward" => (amount, 0),
                "up" => (0, -amount),
                "down" => (0, amount),
                _ => panic!("Unexpected instruction {}", direction)
            }
        })
        .fold((0, 0), |prev, current| (prev.0 + current.0, prev.1 + current.1))
}

fn final_position_part2(steps: &Vec<String>) -> (i32, i32) {
    let steps_re = Regex::new(r#"(forward|down|up) (\d+)"#).unwrap();

    let (_, _, pos_h, depth, _) = steps.iter()
        .filter(|str| !str.is_empty())
        .map(|step| {
            let captures = steps_re.captures(step).unwrap();
            let direction = captures.get(1).unwrap().as_str();
            let amount = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (direction, amount, 0, 0, 0)
        })
        .fold(("", 0, 0, 0, 0), |(_, _, prev_h, prev_depth, prev_aim), (step, amount, _, _, _)| match step {
            "up" => (step, 0, prev_h, prev_depth, prev_aim - amount),
            "down" => (step, 0, prev_h, prev_depth, prev_aim + amount),
            "forward" => (step, 0, prev_h + amount, prev_depth + (amount * prev_aim), prev_aim),
            _ => panic!("Unexpected instruction {}", step)
        });

    (pos_h, depth)
}

#[cfg(test)]
mod tests {
    use crate::{final_position_part1, final_position_part2};

    #[test]  // Part 1
    fn test_basic_part1() {
        let steps = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let (horizontal, vertical) = final_position_part1(&steps);
        assert_eq!(horizontal * vertical, 150);
    }

    #[test]  // Part 1 - with data
    fn test_part1() {
        let steps: Vec<String> = util::read_lines("./src/data.txt");

        let (horizontal, vertical) = final_position_part1(&steps);
        assert_eq!(horizontal * vertical, 1893605);
    }

    #[test]  // Part 2
    fn test_basic_part2() {
        let steps = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let (horizontal, vertical) = final_position_part2(&steps);
        assert_eq!(horizontal * vertical, 900);
    }

    #[test]  // Part 2 - with data
    fn test_part2() {
        let steps: Vec<String> = util::read_lines("./src/data.txt");

        let (horizontal, vertical) = final_position_part2(&steps);
        assert_eq!(horizontal * vertical, 2120734350);
    }
}
