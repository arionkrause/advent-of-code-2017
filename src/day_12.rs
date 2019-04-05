use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split("<-> ").collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|program| program.parse().unwrap())
                .collect()
        })
        .collect()
}

mod part_1 {
    use crate::day_12::decode_input;
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        let groups = decode_input(&input);
        let mut programs_in_group_0 = 0;
        let mut queue = vec![0];
        let mut seem = HashSet::new();

        while let Some(program) = queue.pop() {
            seem.insert(program);

            for connected_program in groups[program].iter() {
                if !queue.contains(connected_program) && !seem.contains(connected_program) {
                    queue.push(*connected_program);
                }
            }

            programs_in_group_0 += 1;
        }

        programs_in_group_0
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(solve(input), 6);
    }
}
