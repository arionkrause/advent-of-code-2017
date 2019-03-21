use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> u32 {
        let characters: Vec<char> = input.chars().collect();

        (0..characters.len()).fold(0, |sum, i| {
            if characters[i] == characters[if i == characters.len() - 1 { 0 } else { i + 1 }] {
                sum + characters[i].to_digit(10).unwrap()
            } else {
                sum
            }
        })
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("1122"), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("1111"), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("1234"), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("91212129"), 9);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> u32 {
        let characters: Vec<char> = input.chars().collect();

        (0..characters.len()).fold(0, |sum, i| {
            if characters[i] == characters[(i + characters.len() / 2) % characters.len()] {
                sum + characters[i].to_digit(10).unwrap()
            } else {
                sum
            }
        })
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("1212"), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("1221"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("123425"), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("123123"), 12);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("12131415"), 4);
    }
}
