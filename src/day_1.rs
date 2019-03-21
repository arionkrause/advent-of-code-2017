use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> u32 {
        let mut sum = 0;
        let characters: Vec<char> = input.chars().collect();

        for i in 0..characters.len() - 1 {
            if characters[i] == characters[i + 1] {
                sum += characters[i].to_digit(10).unwrap();
            }
        }

        if characters[characters.len() - 1] == characters[0] {
            sum += characters[characters.len() - 1].to_digit(10).unwrap();
        }

        sum
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
