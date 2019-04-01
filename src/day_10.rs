use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input, 256));
    println!();
}

mod part_1 {
    use std::collections::VecDeque;

    pub fn solve(input: &str, list_size: usize) -> usize {
        let lengths = get_lengths(&input);
        let mut list = get_list(list_size);
        let mut current_position = 0;

        for (skip_size, length) in lengths.iter().enumerate() {
            for _ in 0..current_position {
                if let Some(popped_front) = list.pop_front() {
                    list.push_back(popped_front);
                }
            }

            let mut popped_values = Vec::new();

            for _ in 0..*length {
                popped_values.push(list.pop_front().unwrap());
            }

            popped_values.reverse();
            let items = popped_values.len();

            for _ in 0..items {
                list.push_front(popped_values.pop().unwrap());
            }

            for _ in 0..current_position {
                if let Some(popped_back) = list.pop_back() {
                    list.push_front(popped_back);
                }
            }

            current_position = (current_position + length + skip_size) % list.len();
        }

        list[0] * list[1]
    }

    fn get_lengths(input: &str) -> Vec<usize> {
        input
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn get_list(list_size: usize) -> VecDeque<usize> {
        let mut list = VecDeque::with_capacity(list_size);

        for i in 0..list_size {
            list.push_back(i);
        }

        list
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("3,4,1,5", 5), 12);
    }
}
