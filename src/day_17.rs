use super::print_day;
use std::collections::vec_deque::VecDeque;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn rotate_right(deque: &mut VecDeque<usize>, amount: usize) {
    for _ in 0..amount {
        if let Some(popped) = deque.pop_back() {
            deque.push_front(popped);
        }
    }
}

mod part_1 {
    use crate::day_17::rotate_right;
    use std::collections::vec_deque::VecDeque;

    pub fn solve(input: &str) -> usize {
        let amount_steps_per_insert: usize = input.parse().unwrap();
        let mut circular_buffer = VecDeque::new();
        circular_buffer.push_front(0);
        circular_buffer.push_front(1);

        for number in 2..=2017 {
            rotate_right(&mut circular_buffer, amount_steps_per_insert);
            circular_buffer.push_front(number);
        }

        rotate_right(&mut circular_buffer, 1);
        circular_buffer.pop_front().unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("3"), 638);
    }
}

mod part_2 {
    use crate::day_17::rotate_right;
    use std::collections::vec_deque::VecDeque;

    pub fn solve(input: &str) -> usize {
        let amount_steps_per_insert: usize = input.parse().unwrap();
        let mut circular_buffer = VecDeque::new();
        circular_buffer.push_front(0);
        circular_buffer.push_front(1);

        for number in 2..=50_000_000 {
            rotate_right(&mut circular_buffer, amount_steps_per_insert);
            circular_buffer.push_front(number);
        }

        let position = circular_buffer
            .iter()
            .position(|number| *number == 0)
            .unwrap();

        circular_buffer[position - 1]
    }
}
