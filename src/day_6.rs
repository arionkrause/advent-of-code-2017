use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        let mut banks: Vec<usize> = input
            .split_whitespace()
            .map(|slice| slice.parse().unwrap())
            .collect();

        let mut seem_configurations = HashSet::new();
        seem_configurations.insert(banks.clone());
        let mut cycle = 1;

        loop {
            let max_bank = banks.iter().max().unwrap();

            let index_max = banks
                .iter()
                .enumerate()
                .filter(|&(_, bank)| bank == max_bank)
                .min()
                .unwrap()
                .0;

            let mut distributable = banks[index_max];
            banks[index_max] = 0;
            let mut index = (index_max + 1) % banks.len();

            while distributable > 0 {
                distributable -= 1;
                banks[index] += 1;
                index = (index + 1) % banks.len();
            }

            match seem_configurations.get(&banks) {
                Some(_) => return cycle,
                None => {
                    seem_configurations.insert(banks.clone());
                }
            }

            cycle += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("0 2   7   0"), 5);
    }
}

mod part_2 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut banks: Vec<usize> = input
            .split_whitespace()
            .map(|slice| slice.parse().unwrap())
            .collect();

        let mut seem_configurations = HashMap::new();
        let mut cycle = 1;
        seem_configurations.insert(banks.clone(), cycle);

        loop {
            let max_bank = banks.iter().max().unwrap();

            let index_max = banks
                .iter()
                .enumerate()
                .filter(|&(_, bank)| bank == max_bank)
                .min()
                .unwrap()
                .0;

            let mut distributable = banks[index_max];
            banks[index_max] = 0;
            let mut index = (index_max + 1) % banks.len();

            while distributable > 0 {
                distributable -= 1;
                banks[index] += 1;
                index = (index + 1) % banks.len();
            }

            match seem_configurations.get(&banks) {
                Some(seem_cycle) => return cycle - *seem_cycle,
                None => {
                    seem_configurations.insert(banks.clone(), cycle);
                }
            }

            cycle += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("0 2   7   0"), 4);
    }
}
