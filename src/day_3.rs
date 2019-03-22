use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let input_as_number: usize = input.parse().unwrap();
        let grid_side_size = get_grid_side_size(input_as_number);

        if grid_side_size == 1 {
            return 0;
        }

        let mut smaller_number_on_same_side_as_target = grid_side_size * grid_side_size;

        loop {
            if smaller_number_on_same_side_as_target < input_as_number - grid_side_size {
                break;
            }

            smaller_number_on_same_side_as_target -= grid_side_size - 1;
        }

        smaller_number_on_same_side_as_target += grid_side_size - 1;
        let steps_to_center_element_on_side = (grid_side_size - 1) / 2;
        let center_element_value =
            smaller_number_on_same_side_as_target + ((grid_side_size - 1) / 2);

        steps_to_center_element_on_side
            + ((input_as_number as isize - center_element_value as isize).abs() as usize)
    }

    fn get_grid_side_size(number: usize) -> usize {
        let mut size = 1;

        while size * size < number {
            size += 2;
        }

        size
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("1"), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("12"), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("23"), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("1024"), 31);
    }
}
