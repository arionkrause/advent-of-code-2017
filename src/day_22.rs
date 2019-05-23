use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input, 10000, false));
    println!();
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn decode_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

mod part_1 {
    use crate::day_22::{decode_input, Direction};

    pub fn solve(input: &str, amount_bursts: usize, print_status: bool) -> usize {
        let initial_grid = decode_input(&input);
        let (mut grid, mut current_x, mut current_y) = create_large_grid(&initial_grid);
        let mut direction = Direction::Up;
        let mut amount_new_infections = 0;

        for _ in 0..amount_bursts {
            if grid[current_y][current_x] == '#' {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };

                grid[current_y][current_x] = '.';
            } else {
                direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                grid[current_y][current_x] = '#';
                amount_new_infections += 1;
            }

            match direction {
                Direction::Up => current_y -= 1,
                Direction::Down => current_y += 1,
                Direction::Left => current_x -= 1,
                Direction::Right => current_x += 1,
            };

            if print_status {
                print_grid(&grid);
            }
        }

        amount_new_infections
    }

    fn create_large_grid(initial_grid: &[Vec<char>]) -> (Vec<Vec<char>>, usize, usize) {
        let side_size = 1000;
        let mut grid = vec![vec!['.'; side_size]; side_size];

        for (index_row, row) in initial_grid.iter().enumerate() {
            for (index_node, node) in row.iter().enumerate() {
                grid[side_size / 2 - initial_grid.len() / 2 + index_row]
                    [side_size / 2 - initial_grid.len() / 2 + index_node] = *node;
            }
        }

        (grid, side_size / 2, side_size / 2)
    }

    fn print_grid(grid: &[Vec<char>]) {
        for row in grid.iter() {
            for node in row.iter() {
                print!("{}", node);
            }

            println!();
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "..#
#..
...";

        assert_eq!(solve(input, 70, false), 41);
    }

    #[test]
    fn test_2() {
        let input = "..#
#..
...";

        assert_eq!(solve(input, 7, false), 5);
    }

    #[test]
    fn test_3() {
        let input = "..#
#..
...";

        assert_eq!(solve(input, 10000, false), 5587);
    }
}
