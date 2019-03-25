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

mod part_2 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| {
                let mut words: Vec<Vec<char>> = line
                    .split_whitespace()
                    .map(|word| word.chars().collect::<Vec<char>>())
                    .collect();

                words.iter_mut().for_each(|word| word.sort());

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
        assert_eq!(solve("abcde fghij"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("abcde xyz ecdab"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("a ab abc abd abf abj"), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("iiii oiii ooii oooi oooo"), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("oiii ioii iioi iiio"), 0);
    }
}
