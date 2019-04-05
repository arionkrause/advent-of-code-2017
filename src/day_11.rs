use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

fn decode_input(input: &str) -> Vec<Direction> {
    input
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&direction| match direction {
            "n" => Direction::North,
            "ne" => Direction::Northeast,
            "se" => Direction::Southeast,
            "s" => Direction::South,
            "sw" => Direction::Southwest,
            "nw" => Direction::Northwest,
            _ => panic!(),
        })
        .collect()
}

mod part_1 {
    use crate::day_11::{decode_input, Direction, Position};

    pub fn solve(input: &str) -> usize {
        let mut position = Position { x: 0, y: 0, z: 0 };

        decode_input(&input).iter().for_each(|step| match step {
            Direction::North => {
                position.y += 1;
                position.z -= 1;
            }
            Direction::Northeast => {
                position.x += 1;
                position.z -= 1;
            }
            Direction::Southeast => {
                position.x += 1;
                position.y -= 1;
            }
            Direction::South => {
                position.y -= 1;
                position.z += 1;
            }
            Direction::Southwest => {
                position.x -= 1;
                position.z += 1;
            }
            Direction::Northwest => {
                position.x -= 1;
                position.y += 1;
            }
        });

        ((position.x.abs() + position.y.abs() + position.z.abs()) / 2) as usize
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("ne,ne,ne"), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("ne,ne,sw,sw"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("ne,ne,s,s"), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("se,sw,se,sw,sw"), 3);
    }
}

mod part_2 {
    use crate::day_11::{decode_input, Direction, Position};

    pub fn solve(input: &str) -> usize {
        let mut position = Position { x: 0, y: 0, z: 0 };
        let mut maximum_amount_steps = 0;

        decode_input(&input).iter().for_each(|step| {
            match step {
                Direction::North => {
                    position.y += 1;
                    position.z -= 1;
                }
                Direction::Northeast => {
                    position.x += 1;
                    position.z -= 1;
                }
                Direction::Southeast => {
                    position.x += 1;
                    position.y -= 1;
                }
                Direction::South => {
                    position.y -= 1;
                    position.z += 1;
                }
                Direction::Southwest => {
                    position.x -= 1;
                    position.z += 1;
                }
                Direction::Northwest => {
                    position.x -= 1;
                    position.y += 1;
                }
            };

            let amount_steps =
                ((position.x.abs() + position.y.abs() + position.z.abs()) / 2) as usize;

            if amount_steps > maximum_amount_steps {
                maximum_amount_steps = amount_steps;
            }
        });

        maximum_amount_steps
    }
}
