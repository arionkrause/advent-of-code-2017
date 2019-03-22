use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let mut numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|slice| slice.parse::<usize>().unwrap())
                    .collect();

                numbers.sort();
                numbers[numbers.len() - 1] - numbers[0]
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

mod part_2 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|slice| slice.parse::<usize>().unwrap())
                    .collect();

                for (index, i) in numbers.iter().take(numbers.len() - 1).enumerate() {
                    for j in numbers.iter().skip(index + 1) {
                        if i % j == 0 {
                            return i / j;
                        }

                        if j % i == 0 {
                            return j / i;
                        }
                    }
                }

                panic!();
            })
            .sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "5	9	2	8
9	4	7	3
3	8	6	5";

        assert_eq!(solve(&input), 9);
    }
}
