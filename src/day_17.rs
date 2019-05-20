use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let amount_steps_per_insert: usize = input.parse().unwrap();
        let mut circular_buffer = Vec::new();
        circular_buffer.push(0);
        let mut position = 0;

        for number in 1..=2017 {
            position = (position + amount_steps_per_insert) % circular_buffer.len();

            if position == circular_buffer.len() - 1 {
                circular_buffer.push(number);
                position = circular_buffer.len() - 1;
            } else {
                position += 1;
                circular_buffer.insert(position, number);
            }
        }

        position += 1;

        if position == circular_buffer.len() {
            position = 0;
        }

        circular_buffer[position]
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("3"), 638);
    }
}
