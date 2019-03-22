use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|slice| slice.parse::<u32>().unwrap())
                    .max()
                    .unwrap()
                    - line
                        .split_whitespace()
                        .map(|slice| slice.parse::<u32>().unwrap())
                        .min()
                        .unwrap()
            })
            .sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "5  1   9   5
7   5   3
2   4   6   8";

        assert_eq!(solve(&input), 18);
    }
}
