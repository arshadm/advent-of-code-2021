use num_bigint::BigUint;
use num_traits::Zero;

fn main() {
    println!("Hello, world!");
}

fn compute_lanterfish_after_days(lanterfish: &mut Vec<BigUint>, num_days: i32) -> BigUint {
    for day in 1..=num_days {
        let age_0 = lanterfish.get(0).unwrap().clone();
        print!("Day {}, {:?}\n", day, *lanterfish);

        for i in 0..8 {
            lanterfish[i] = lanterfish[i + 1].clone();
        }

        lanterfish[6] = &lanterfish[6] + &age_0;
        lanterfish[8] = age_0;
    }

    lanterfish.iter().fold(BigUint::zero(), |a, b| -> BigUint { a + b })
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    use crate::{compute_lanterfish_after_days};

    #[test]  // Part 1 - small sample
    fn test_lanterfish_part1() {
        let fish = vec!["3,4,3,1,2".to_string()];
        let mut fish_by_age = vec![Zero::zero(); 9];

        compute_byage(fish, &mut fish_by_age);

        let num_fish = compute_lanterfish_after_days(&mut fish_by_age, 80);
        print!("{:?}", num_fish);
    }

    #[test]  // Part 1
    fn test_lanterfish_part1_with_data() {
        let fish = vec!["5,1,2,1,5,3,1,1,1,1,1,2,5,4,1,1,1,1,2,1,2,1,1,1,1,1,2,1,5,1,1,1,3,1,1,1,3,1,1,3,1,1,4,3,1,1,4,1,1,1,1,2,1,1,1,5,1,1,5,1,1,1,4,4,2,5,1,1,5,1,1,2,2,1,2,1,1,5,3,1,2,1,1,3,1,4,3,3,1,1,3,1,5,1,1,3,1,1,4,4,1,1,1,5,1,1,1,4,4,1,3,1,4,1,1,4,5,1,1,1,4,3,1,4,1,1,4,4,3,5,1,2,2,1,2,2,1,1,1,2,1,1,1,4,1,1,3,1,1,2,1,4,1,1,1,1,1,1,1,1,2,2,1,1,5,5,1,1,1,5,1,1,1,1,5,1,3,2,1,1,5,2,3,1,2,2,2,5,1,1,3,1,1,1,5,1,4,1,1,1,3,2,1,3,3,1,3,1,1,1,1,1,1,1,2,3,1,5,1,4,1,3,5,1,1,1,2,2,1,1,1,1,5,4,1,1,3,1,2,4,2,1,1,3,5,1,1,1,3,1,1,1,5,1,1,1,1,1,3,1,1,1,4,1,1,1,1,2,2,1,1,1,1,5,3,1,2,3,4,1,1,5,1,2,4,2,1,1,1,2,1,1,1,1,1,1,1,4,1,5".to_string()];
        let mut fish_by_age = vec![Zero::zero(); 9];

        compute_byage(fish, &mut fish_by_age);

        let num_fish = compute_lanterfish_after_days(&mut fish_by_age, 80);
        print!("{:?}", num_fish);
    }

    #[test]  // Part 2
    fn test_lanterfish_part2_with_data() {
        let fish = vec!["5,1,2,1,5,3,1,1,1,1,1,2,5,4,1,1,1,1,2,1,2,1,1,1,1,1,2,1,5,1,1,1,3,1,1,1,3,1,1,3,1,1,4,3,1,1,4,1,1,1,1,2,1,1,1,5,1,1,5,1,1,1,4,4,2,5,1,1,5,1,1,2,2,1,2,1,1,5,3,1,2,1,1,3,1,4,3,3,1,1,3,1,5,1,1,3,1,1,4,4,1,1,1,5,1,1,1,4,4,1,3,1,4,1,1,4,5,1,1,1,4,3,1,4,1,1,4,4,3,5,1,2,2,1,2,2,1,1,1,2,1,1,1,4,1,1,3,1,1,2,1,4,1,1,1,1,1,1,1,1,2,2,1,1,5,5,1,1,1,5,1,1,1,1,5,1,3,2,1,1,5,2,3,1,2,2,2,5,1,1,3,1,1,1,5,1,4,1,1,1,3,2,1,3,3,1,3,1,1,1,1,1,1,1,2,3,1,5,1,4,1,3,5,1,1,1,2,2,1,1,1,1,5,4,1,1,3,1,2,4,2,1,1,3,5,1,1,1,3,1,1,1,5,1,1,1,1,1,3,1,1,1,4,1,1,1,1,2,2,1,1,1,1,5,3,1,2,3,4,1,1,5,1,2,4,2,1,1,1,2,1,1,1,1,1,1,1,4,1,5".to_string()];
        let mut fish_by_age = vec![Zero::zero(); 9];

        compute_byage(fish, &mut fish_by_age);

        let num_fish = compute_lanterfish_after_days(&mut fish_by_age, 256);
        print!("{:?}", num_fish);
    }

    ///
    /// Compute a count of fish by age
    ///
    fn compute_byage(fish: Vec<String>, fish_by_age: &mut Vec<BigUint>) {
        fish.get(0).unwrap().split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<i32>().unwrap())
            .for_each(|age| {
                let one: BigUint = One::one();
                fish_by_age[age as usize] = one + &fish_by_age[age as usize];
            });
    }
}
