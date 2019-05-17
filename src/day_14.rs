use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::vec_deque::VecDeque;

    pub fn solve(input: &str) -> u32 {
        (0..128)
            .map(|row| {
                let hash_input = format!("{}-{}", input, row);
                let knot_hash = get_knot_hash(&hash_input);
                let knot_hash_in_decimal = u128::from_str_radix(&knot_hash, 16).unwrap();
                knot_hash_in_decimal.count_ones()
            })
            .sum()
    }

    pub fn get_knot_hash(input: &str) -> String {
        let lengths = get_lengths(&input);
        let mut list = get_list();
        let mut current_position = 0;
        let mut skip_size = 0;

        for _ in 0..64 {
            for length in lengths.iter() {
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

                current_position = (current_position + *length as usize + skip_size) % list.len();
                skip_size += 1;
            }
        }

        (0..16)
            .map(|i| {
                let mut output_number = list[i * 16];

                for j in 1..16 {
                    output_number ^= list[i * 16 + j];
                }

                format!("{:02x}", output_number)
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_lengths(input: &str) -> Vec<u8> {
        let mut lengths: Vec<u8> = input.chars().map(|character| character as u8).collect();
        lengths.extend(vec![17, 31, 73, 47, 23]);
        lengths
    }

    fn get_list() -> VecDeque<u8> {
        let mut list = VecDeque::with_capacity(255);

        for i in 0..=255 {
            list.push_back(i);
        }

        list
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("flqrgnkx"), 8108);
    }
}
