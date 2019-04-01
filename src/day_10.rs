use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input, 255));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::VecDeque;

    pub fn solve(input: &str, list_max_value: u8) -> usize {
        let lengths = get_lengths(&input);
        let mut list = get_list(list_max_value);
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

        list[0] as usize * list[1] as usize
    }

    fn get_lengths(input: &str) -> Vec<usize> {
        input
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn get_list(list_max_value: u8) -> VecDeque<u8> {
        let mut list = VecDeque::with_capacity(list_max_value as usize);

        for i in 0..=list_max_value {
            list.push_back(i);
        }

        list
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("3,4,1,5", 4), 12);
    }
}

mod part_2 {
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> String {
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
        assert_eq!(solve(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
