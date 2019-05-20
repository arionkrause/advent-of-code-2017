use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> (usize, usize) {
    let re = Regex::new(r"^Generator A starts with (?P<starting_value_a>\d+)\nGenerator B starts with (?P<starting_value_b>\d+)$").unwrap();
    let captures = re.captures(&input).unwrap();
    (
        captures
            .name("starting_value_a")
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        captures
            .name("starting_value_b")
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
    )
}

mod part_1 {
    use crate::day_15::decode_input;

    pub fn solve(input: &str) -> usize {
        let (mut value_a, mut value_b) = decode_input(&input);
        let mut amount_matches = 0;

        for _ in 0..40_000_000 {
            value_a = value_a * 16807 % 0x7FFF_FFFF;
            value_b = value_b * 48271 % 0x7FFF_FFFF;

            if value_a & 0xFFFF == value_b & 0xFFFF {
                amount_matches += 1;
            }
        }

        amount_matches
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(solve(&input), 588);
    }
}

mod part_2 {
    use crate::day_15::decode_input;

    pub fn solve(input: &str) -> usize {
        let (mut value_a, mut value_b) = decode_input(&input);
        let mut amount_matches = 0;

        for _ in 0..5_000_000 {
            value_a = get_next_value(value_a, 16807, 4);
            value_b = get_next_value(value_b, 48271, 8);

            if value_a & 0xFFFF == value_b & 0xFFFF {
                amount_matches += 1;
            }
        }

        amount_matches
    }

    fn get_next_value(current_value: usize, factor: usize, multiple_of: usize) -> usize {
        let mut new_value = current_value;

        loop {
            new_value = new_value * factor % 0x7FFF_FFFF;

            if new_value % multiple_of == 0 {
                return new_value;
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(solve(&input), 309);
    }
}
