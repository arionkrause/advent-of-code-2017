use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Clone, Debug)]
struct State {
    position: Position,
    direction: Direction,
    collected_letters: Vec<char>,
}

#[derive(Clone, Debug)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn decode_input(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let max_row_length = grid.iter().map(std::vec::Vec::len).max().unwrap();

    grid.iter_mut().for_each(|row| {
        while row.len() < max_row_length {
            row.push(' ');
        }
    });

    grid
}

mod part_1 {
    use crate::day_19::{decode_input, Direction, Position, State};
    use std::collections::vec_deque::VecDeque;

    pub fn solve(input: &str) -> String {
        let grid = decode_input(&input);
        let starting_x = grid[0].iter().position(|tile| *tile == '|').unwrap();

        let state = State {
            position: Position {
                y: 0,
                x: starting_x,
            },
            direction: Direction::Down,
            collected_letters: vec![],
        };

        let mut queue = VecDeque::new();
        queue.push_front(state);

        loop {
            let mut state = queue.pop_front().unwrap();
            //            print_status(&grid, &state);

            if grid[state.position.y][state.position.x].is_alphabetic() {
                state
                    .collected_letters
                    .push(grid[state.position.y][state.position.x]);
            }

            let new_moves = get_new_moves(&grid, &state);

            if new_moves.is_empty() {
                return state.collected_letters.iter().collect();
            }

            let mut continued = false;

            match state.direction {
                Direction::Up => {
                    if state.position.y > 0 && grid[state.position.y - 1][state.position.x] != ' ' {
                        continued = true;
                        state.position.y -= 1;
                    }
                }
                Direction::Down => {
                    if state.position.y < grid.len() - 1
                        && grid[state.position.y + 1][state.position.x] != ' '
                    {
                        continued = true;
                        state.position.y += 1;
                    }
                }
                Direction::Left => {
                    if state.position.x > 0 && grid[state.position.y][state.position.x - 1] != ' ' {
                        continued = true;
                        state.position.x -= 1;
                    }
                }
                Direction::Right => {
                    if state.position.x < grid.iter().map(std::vec::Vec::len).max().unwrap() - 1
                        && grid[state.position.y][state.position.x + 1] != ' '
                    {
                        continued = true;
                        state.position.x += 1;
                    }
                }
            };

            if continued {
                queue.push_front(state);
            } else {
                for (new_position, new_direction) in new_moves {
                    let mut state_clone = state.clone();
                    state_clone.position = new_position;
                    state_clone.direction = new_direction;
                    queue.push_front(state_clone);
                }
            }
        }
    }

    fn get_new_moves(grid: &[Vec<char>], state: &State) -> Vec<(Position, Direction)> {
        let mut new_moves = Vec::new();

        if (state.direction == Direction::Up
            || state.direction == Direction::Left
            || state.direction == Direction::Right)
            && state.position.y > 0
            && grid[state.position.y - 1][state.position.x] != ' '
        {
            new_moves.push((
                Position {
                    y: state.position.y - 1,
                    x: state.position.x,
                },
                Direction::Up,
            ));
        }

        if (state.direction == Direction::Down
            || state.direction == Direction::Left
            || state.direction == Direction::Right)
            && state.position.y < grid.len() - 1
            && grid[state.position.y + 1][state.position.x] != ' '
        {
            new_moves.push((
                Position {
                    y: state.position.y + 1,
                    x: state.position.x,
                },
                Direction::Down,
            ));
        }

        if (state.direction == Direction::Up
            || state.direction == Direction::Left
            || state.direction == Direction::Down)
            && state.position.x > 0
            && grid[state.position.y][state.position.x - 1] != ' '
        {
            new_moves.push((
                Position {
                    y: state.position.y,
                    x: state.position.x - 1,
                },
                Direction::Left,
            ));
        }

        if (state.direction == Direction::Up
            || state.direction == Direction::Right
            || state.direction == Direction::Down)
            && state.position.x < grid.iter().map(std::vec::Vec::len).max().unwrap() - 1
            && grid[state.position.y][state.position.x + 1] != ' '
        {
            new_moves.push((
                Position {
                    y: state.position.y,
                    x: state.position.x + 1,
                },
                Direction::Right,
            ));
        }

        new_moves
    }

    //    fn print_status(grid: &[Vec<char>], state: &State) {
    //        for (index_row, row) in grid.iter().enumerate() {
    //            for (index_tile, tile) in row.iter().enumerate() {
    //                if state.position.y == index_row && state.position.x == index_tile {
    //                    eprint!("o");
    //                } else {
    //                    eprint!("{}", tile);
    //                }
    //            }
    //
    //            eprintln!();
    //        }
    //
    //        eprintln!("state.position = {:?}", state.position);
    //        eprintln!("state.direction = {:?}", state.direction);
    //        eprintln!("state.collected_letters = {:?}", state.collected_letters);
    //        eprintln!();
    //    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let mut input = String::from("");
        input.push_str("    |         \n");
        input.push_str("    |  +--+   \n");
        input.push_str("    A  |  C   \n");
        input.push_str("F---|----E|--+\n");
        input.push_str("    |  |  |  D\n");
        input.push_str("    +B-+  +--+\n");
        assert_eq!(solve(&input), "ABCDEF");
    }
}
