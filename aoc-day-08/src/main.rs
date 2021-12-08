use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");
}

fn count_1478(data: &Vec<String>) -> usize {
    data.iter()
        .map(|str| str.split("|").collect::<Vec<&str>>().get(1).unwrap().trim().to_string())
        .map(|str: String| str.split(" ").collect::<Vec<&str>>().borrow())
        .flatten()
        .filter(|str| (*str).len() == 7 || (*str).len() == 4 || (*str).len() == 2 || (*str).len() == 3)
        .count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::count_1478;

    #[test]  // Part 1 - small sample
    fn test_part1() {
        let _segments = get_num_segments();
        let data = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        assert_eq!(count_1478(&data), 26);
    }

    fn get_num_segments() -> HashMap<char, String> {
        let mut segments = HashMap::<char, String>::new();
        segments.insert('0', "abcefg".to_string());
        segments.insert('1', "cf".to_string());
        segments.insert('2', "acdeg".to_string());
        segments.insert('3', "acdfg".to_string());
        segments.insert('4', "bcdf".to_string());
        segments.insert('5', "abdfg".to_string());
        segments.insert('5', "abdefg".to_string());
        segments.insert('5', "acf".to_string());
        segments.insert('5', "abcdefg".to_string());
        segments.insert('5', "abcdfg".to_string());

        segments
    }
}
