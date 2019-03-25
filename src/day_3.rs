use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_grid_side_size(number: usize) -> usize {
    let mut size = 1;

    while size * size < number {
        size += 2;
    }

    size
}

mod part_1 {
    use crate::day_3::get_grid_side_size;

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

mod part_2 {
    use crate::day_3::get_grid_side_size;

    #[derive(Eq, PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub fn solve(input: &str) -> usize {
        let input_as_number: usize = input.parse().unwrap();
        let grid_side_size = get_grid_side_size(input_as_number); // Not optimal but will suffice
        let mut grid = vec![vec![0; grid_side_size]; grid_side_size];
        let mut x = grid_side_size / 2 as usize;
        let mut y = grid_side_size / 2 as usize;
        grid[y][x] = 1;
        x += 1;
        let mut direction = Direction::Up;
        let mut steps = 2;

        'side: loop {
            let mut steps_remaining = steps;

            while steps_remaining > 0 {
                let value = get_value(&grid, x, y);

                if value > input_as_number {
                    return value;
                }

                grid[y][x] = value;
                steps_remaining -= 1;

                if steps_remaining == 0 {
                    match direction {
                        Direction::Up => {
                            direction = Direction::Left;
                            x -= 1;
                        }
                        Direction::Down => {
                            direction = Direction::Right;
                            x += 1;
                        }
                        Direction::Left => {
                            direction = Direction::Down;
                            y += 1;
                        }
                        Direction::Right => {
                            direction = Direction::Up;
                            x += 1;
                            steps += 2;
                            continue 'side;
                        }
                    }
                } else {
                    match direction {
                        Direction::Up => y -= 1,
                        Direction::Down => y += 1,
                        Direction::Left => x -= 1,
                        Direction::Right => x += 1,
                    }
                }
            }
        }
    }

    fn get_value(grid: &[Vec<usize>], x: usize, y: usize) -> usize {
        let mut value = 0;

        if y > 0 && x > 0 {
            value += grid[y - 1][x - 1];
        }

        if y > 0 {
            value += grid[y - 1][x];
        }

        if y > 0 && x < grid[0].len() - 1 {
            value += grid[y - 1][x + 1];
        }

        if x > 0 {
            value += grid[y][x - 1];
        }

        if x < grid[0].len() - 1 {
            value += grid[y][x + 1];
        }

        if y < grid.len() - 1 && x > 0 {
            value += grid[y + 1][x - 1];
        }

        if y < grid.len() - 1 {
            value += grid[y + 1][x];
        }

        if y < grid.len() - 1 && x < grid[0].len() - 1 {
            value += grid[y + 1][x + 1];
        }

        value
    }
}
