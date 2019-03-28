use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let mut characters = input.chars();
        let mut score = 0;
        let mut score_level = 0;
        let mut garbage = false;

        while let Some(character) = characters.next() {
            if character == '!' {
                characters.next();
            } else if garbage {
                if character == '>' {
                    garbage = false;
                }
            } else if character == '<' {
                garbage = true;
            } else if character == '{' {
                score_level += 1;
            } else if character == '}' {
                score += score_level;
                score_level -= 1;
            }
        }

        score
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("<>"), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("<random characters>"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("<<<<>"), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("<{!>}>"), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("<!!>"), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve("<!!!>>"), 0);
    }

    #[test]
    fn test_7() {
        assert_eq!(solve(r#"<{o"i!a,<{i<a>"#), 0);
    }

    #[test]
    fn test_8() {
        assert_eq!(solve("{}"), 1);
    }

    #[test]
    fn test_9() {
        assert_eq!(solve("{{{}}}"), 6);
    }

    #[test]
    fn test_10() {
        assert_eq!(solve("{{},{}}"), 5);
    }

    #[test]
    fn test_11() {
        assert_eq!(solve("{{{},{},{{}}}}"), 16);
    }

    #[test]
    fn test_12() {
        assert_eq!(solve("{<a>,<a>,<a>,<a>}"), 1);
    }

    #[test]
    fn test_13() {
        assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }

    #[test]
    fn test_14() {
        assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }

    #[test]
    fn test_15() {
        assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
