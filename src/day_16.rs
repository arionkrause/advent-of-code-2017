use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input, "abcdefghijklmnop"));
    println!();
}

mod part_1 {
    use regex::Regex;
    use std::collections::vec_deque::VecDeque;

    pub fn solve(input: &str, programs_input: &str) -> String {
        let mut programs: VecDeque<char> = programs_input.chars().collect();
        let re_spin = Regex::new(r"s(?P<amount>\d+)").unwrap();
        let re_exchange = Regex::new(r"x(?P<position_a>\d+)/(?P<position_b>\d+)").unwrap();
        let re_partner = Regex::new(r"p(?P<program_a>[a-p])/(?P<program_b>[a-p])").unwrap();

        for slice in input.split(',') {
            if let Some(captures) = re_spin.captures(slice) {
                let amount = captures.name("amount").unwrap().as_str().parse().unwrap();
                rotate_right(&mut programs, amount);
                continue;
            }

            if let Some(captures) = re_exchange.captures(slice) {
                let position_a = captures
                    .name("position_a")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                let position_b = captures
                    .name("position_b")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                programs.swap(position_a, position_b);
                continue;
            }

            if let Some(captures) = re_partner.captures(slice) {
                let program_a = captures
                    .name("program_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let program_b = captures
                    .name("program_b")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let position_a = programs.iter().position(|item| item == &program_a).unwrap();
                let position_b = programs.iter().position(|item| item == &program_b).unwrap();
                programs.swap(position_a, position_b);
                continue;
            }
        }

        programs.iter().collect()
    }

    fn rotate_right(deque: &mut VecDeque<char>, amount: usize) {
        for _ in 0..amount {
            if let Some(popped_back) = deque.pop_back() {
                deque.push_front(popped_back);
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("s1,x3/4,pe/b", "abcde"), "baedc");
    }
}
