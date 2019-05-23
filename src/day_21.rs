use super::print_day;
use regex::Regex;

// I did it the hard way BECAUSE ¯\_(ツ)_/¯
pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input, 5, false));
    println!("Part 2: {}.", part_2::solve(&input, 18, false));
    println!();
}

#[derive(Clone, Debug)]
struct Rule2By2 {
    from: Vec<char>,
    to: Vec<char>,
}

impl Rule2By2 {
    fn matches(&self, block: &[char]) -> bool {
        self.do_match(&block)
            || self.do_match(&Rule2By2::flipped(&block))
            || self.do_match_rotated(&block)
            || self.do_match_rotated(&Rule2By2::flipped(&block))
    }

    fn do_match(&self, block: &[char]) -> bool {
        self.from
            .iter()
            .zip(block.iter())
            .all(|tuple| tuple.0 == tuple.1)
    }

    fn flipped(block: &[char]) -> Vec<char> {
        vec![block[1], block[0], block[3], block[2]]
    }

    fn do_match_rotated(&self, block: &[char]) -> bool {
        let mut rotated_block = block.to_owned();

        for _ in 0..3 {
            Rule2By2::rotate_90_degrees(&mut rotated_block);

            if self.do_match(&rotated_block) {
                return true;
            }
        }

        false
    }

    fn rotate_90_degrees(block: &mut Vec<char>) {
        let aux = block[0];
        block[0] = block[2];
        block[2] = block[3];
        block[3] = block[1];
        block[1] = aux;
    }
}

#[derive(Clone, Debug)]
struct Rule3By3 {
    from: Vec<char>,
    to: Vec<char>,
}

impl Rule3By3 {
    fn matches(&self, block: &[char]) -> bool {
        self.do_match(&block)
            || self.do_match(&Rule3By3::flipped(&block))
            || self.do_match_rotated(&block)
            || self.do_match_rotated(&Rule3By3::flipped(&block))
    }

    fn do_match(&self, block: &[char]) -> bool {
        self.from
            .iter()
            .zip(block.iter())
            .all(|tuple| tuple.0 == tuple.1)
    }

    fn flipped(block: &[char]) -> Vec<char> {
        vec![
            block[2], block[1], block[0], block[5], block[4], block[3], block[8], block[7],
            block[6],
        ]
    }

    fn do_match_rotated(&self, block: &[char]) -> bool {
        let mut rotated_block = block.to_owned();

        for _ in 0..3 {
            Rule3By3::rotate_90_degrees(&mut rotated_block);

            if self.do_match(&rotated_block) {
                return true;
            }
        }

        false
    }

    fn rotate_90_degrees(block: &mut Vec<char>) {
        let mut aux = block[0];
        block[0] = block[6];
        block[6] = block[8];
        block[8] = block[2];
        block[2] = aux;

        aux = block[1];
        block[1] = block[3];
        block[3] = block[7];
        block[7] = block[5];
        block[5] = aux;
    }
}

fn decode_input(input: &str) -> (Vec<Rule2By2>, Vec<Rule3By3>) {
    let re_2_by_2 = Regex::new(r"^([.#]{2})/([.#]{2}) => ([.#]{3})/([.#]{3})/([.#]{3})$").unwrap();
    let re_3_by_3 =
        Regex::new(r"^([.#]{3})/([.#]{3})/([.#]{3}) => ([.#]{4})/([.#]{4})/([.#]{4})/([.#]{4})$")
            .unwrap();
    let mut rules_2_by_2 = Vec::new();
    let mut rules_3_by_3 = Vec::new();

    for line in input.lines() {
        if let Some(captures) = re_2_by_2.captures(&line) {
            let mut from = captures
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>();

            from.extend(
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            let mut to = captures
                .get(3)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>();

            to.extend(
                captures
                    .get(4)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            to.extend(
                captures
                    .get(5)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            rules_2_by_2.push(Rule2By2 { from, to });
            continue;
        }

        if let Some(captures) = re_3_by_3.captures(&line) {
            let mut from = captures
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>();

            from.extend(
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            from.extend(
                captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            let mut to = captures
                .get(4)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>();

            to.extend(
                captures
                    .get(5)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            to.extend(
                captures
                    .get(6)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            to.extend(
                captures
                    .get(7)
                    .unwrap()
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>(),
            );

            rules_3_by_3.push(Rule3By3 { from, to });
            continue;
        }

        panic!();
    }

    (rules_2_by_2, rules_3_by_3)
}

fn run(input: &str, amount_iterations: usize, print_status: bool) -> usize {
    let (rules_2_by_2, rules_3_by_3) = decode_input(&input);
    let mut grid = vec!['.', '#', '.', '.', '.', '#', '#', '#', '#'];

    for _ in 0..amount_iterations {
        if print_status {
            print_grid(&grid);
        }

        if grid.len() % 2 == 0 {
            let mut new_blocks = Vec::new();

            for block in get_blocks_2_by_2(&grid) {
                for rule in &rules_2_by_2 {
                    if rule.matches(&block) {
                        new_blocks.push(rule.to.clone());
                        continue;
                    }
                }
            }

            grid = build_3_by_3_grid(&new_blocks);
        } else {
            let mut new_blocks = Vec::new();

            for block in get_blocks_3_by_3(&grid) {
                for rule in &rules_3_by_3 {
                    if rule.matches(&block) {
                        new_blocks.push(rule.to.clone());
                        continue;
                    }
                }
            }

            grid = build_4_by_4_grid(&new_blocks);
        }
    }

    if print_status {
        print_grid(&grid);
    }

    grid.iter().filter(|&tile| *tile == '#').count()
}

fn get_blocks_2_by_2(grid: &[char]) -> Vec<Vec<char>> {
    let mut blocks = Vec::new();
    let side_size = ((grid.len() / 4) as f32).sqrt() as usize;

    for y in 0..side_size {
        for x in 0..side_size {
            blocks.push(vec![
                grid[y * side_size * 4 + x * 2],
                grid[y * side_size * 4 + x * 2 + 1],
                grid[y * side_size * 4 + x * 2 + side_size * 2],
                grid[y * side_size * 4 + x * 2 + side_size * 2 + 1],
            ]);
        }
    }

    blocks
}

fn get_blocks_3_by_3(grid: &[char]) -> Vec<Vec<char>> {
    let mut blocks = Vec::new();
    let side_size = ((grid.len() / 9) as f32).sqrt() as usize;

    for y in 0..side_size {
        for x in 0..side_size {
            blocks.push(vec![
                grid[y * side_size * 9 + x * 3],
                grid[y * side_size * 9 + x * 3 + 1],
                grid[y * side_size * 9 + x * 3 + 2],
                grid[y * side_size * 9 + x * 3 + side_size * 3],
                grid[y * side_size * 9 + x * 3 + side_size * 3 + 1],
                grid[y * side_size * 9 + x * 3 + side_size * 3 + 2],
                grid[y * side_size * 9 + x * 3 + side_size * 3 * 2],
                grid[y * side_size * 9 + x * 3 + side_size * 3 * 2 + 1],
                grid[y * side_size * 9 + x * 3 + side_size * 3 * 2 + 2],
            ]);
        }
    }

    blocks
}

fn build_4_by_4_grid(blocks: &[Vec<char>]) -> Vec<char> {
    let mut grid = Vec::new();
    let side_size = (blocks.len() as f32).sqrt() as usize;

    for y in 0..side_size {
        for row in 0..4 {
            for x in 0..side_size {
                grid.push(blocks[y * side_size + x][row * 4]);
                grid.push(blocks[y * side_size + x][row * 4 + 1]);
                grid.push(blocks[y * side_size + x][row * 4 + 2]);
                grid.push(blocks[y * side_size + x][row * 4 + 3]);
            }
        }
    }

    grid
}

fn build_3_by_3_grid(blocks: &[Vec<char>]) -> Vec<char> {
    let mut grid = Vec::new();
    let side_size = (blocks.len() as f32).sqrt() as usize;

    for y in 0..side_size {
        for row in 0..3 {
            for x in 0..side_size {
                grid.push(blocks[y * side_size + x][row * 3]);
                grid.push(blocks[y * side_size + x][row * 3 + 1]);
                grid.push(blocks[y * side_size + x][row * 3 + 2]);
            }
        }
    }

    grid
}

fn print_grid(grid: &[char]) {
    if grid.len() % 2 == 0 {
        let side_size = ((grid.len() / 4) as f32).sqrt() as usize;
        eprintln!("grid = {:?}", grid);
        for (index, tile) in grid.iter().enumerate() {
            if index > 0 && index % (side_size * 2) == 0 {
                eprintln!();
            }

            eprint!("{}", tile);
        }
    } else {
        let side_size = ((grid.len() / 9) as f32).sqrt() as usize;

        for (index, tile) in grid.iter().enumerate() {
            if index > 0 && index % (side_size * 3) == 0 {
                eprintln!();
            }

            eprint!("{}", tile);
        }
    }

    eprintln!();
    eprintln!();
}

mod part_1 {
    use crate::day_21::run;

    pub fn solve(input: &str, amount_iterations: usize, print_status: bool) -> usize {
        run(&input, amount_iterations, print_status)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

        assert_eq!(solve(input, 2, true), 12);
    }
}

mod part_2 {
    use crate::day_21::run;

    pub fn solve(input: &str, amount_iterations: usize, print_status: bool) -> usize {
        run(&input, amount_iterations, print_status)
    }
}
