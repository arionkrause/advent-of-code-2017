use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();

                words.iter().all(|word| {
                    words
                        .iter()
                        .filter(|other_word| other_word == &word)
                        .count()
                        == 1
                })
            })
            .count()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("aa bb cc dd ee"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("aa bb cc dd aa"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("aa bb cc dd aaa"), 1);
    }
}
