use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
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
            value_a = value_a * 16807 % 2_147_483_647;
            value_b = value_b * 48271 % 2_147_483_647;

            if value_a & 0b1111_1111_1111_1111 == value_b & 0b1111_1111_1111_1111 {
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
