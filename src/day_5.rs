use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let mut jumps: Vec<isize> = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();

        let mut pc = 0;
        let mut steps = 0;

        loop {
            let new_pc = pc + jumps[pc as usize];
            steps += 1;

            if new_pc >= jumps.len() as isize {
                return steps;
            }

            jumps[pc as usize] += 1;
            pc = new_pc;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0
3
0
1
-3";

        assert_eq!(solve(input), 5);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> usize {
        let mut jumps: Vec<isize> = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();

        let mut pc = 0;
        let mut steps = 0;

        loop {
            let new_pc = pc + jumps[pc as usize];
            steps += 1;

            if new_pc >= jumps.len() as isize {
                return steps;
            }

            if jumps[pc as usize] >= 3 {
                jumps[pc as usize] -= 1;
            } else {
                jumps[pc as usize] += 1;
            }

            pc = new_pc;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0
3
0
1
-3";

        assert_eq!(solve(input), 10);
    }
}
