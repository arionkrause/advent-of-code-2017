use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use regex::Regex;
    use std::collections::HashMap;

    pub fn solve(input: &str) -> String {
        let mut programs = HashMap::new();
        let re =
            Regex::new(r"^(?P<program_name>[a-z]+) \(\d+\)( -> (?P<programs_above>.+))?$").unwrap();

        for line in input.lines() {
            let captures = re.captures(&line).unwrap();
            let program_name = captures.name("program_name").unwrap().as_str();

            if !programs.contains_key(program_name) {
                programs.insert(program_name, false);
            }

            if let Some(programs_above) = captures.name("programs_above") {
                programs_above
                    .as_str()
                    .split(", ")
                    .for_each(|program_above| {
                        programs
                            .entry(&program_above)
                            .and_modify(|has_program_above| *has_program_above = true)
                            .or_insert(true);
                    });
            }
        }

        programs
            .iter()
            .find(|&(_, has_program_above)| !*has_program_above)
            .unwrap()
            .0
            .to_string()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(solve(input), "tknk");
    }
}

mod part_2 {
    use regex::Regex;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Program {
        name: String,
        weight: Option<usize>,
        programs_above: Vec<usize>,
        weight_tower_above: Option<usize>,
    }

    fn decode_input(input: &str) -> Vec<Program> {
        let mut programs: Vec<Program> = Vec::new();

        let re = Regex::new(r"^(?P<name>[a-z]+) \((?P<weight>\d+)\)( -> (?P<programs_above>.+))?$")
            .unwrap();

        for line in input.lines() {
            let captures = re.captures(&line).unwrap();
            let name = captures.name("name").unwrap().as_str().to_string();
            let weight: usize = captures.name("weight").unwrap().as_str().parse().unwrap();

            let index = if let Some((index, program)) = programs
                .iter_mut()
                .enumerate()
                .find(|(_, program)| program.name == name)
            {
                program.weight = Some(weight);
                index
            } else {
                programs.push(Program {
                    name,
                    weight: Some(weight),
                    programs_above: vec![],
                    weight_tower_above: None,
                });

                programs.len() - 1
            };

            if let Some(programs_above) = captures.name("programs_above") {
                programs_above
                    .as_str()
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|name_program_above| {
                        let index_program_above = if let Some((index, _)) = programs
                            .iter_mut()
                            .enumerate()
                            .find(|(_, program)| &program.name == name_program_above)
                        {
                            index
                        } else {
                            programs.push(Program {
                                name: name_program_above.to_string(),
                                weight: None,
                                programs_above: vec![],
                                weight_tower_above: None,
                            });

                            programs.len() - 1
                        };

                        programs[index].programs_above.push(index_program_above);
                    })
            }
        }

        programs
    }

    pub fn solve(input: &str) -> isize {
        let mut programs = decode_input(&input);
        let index_program_bottom = get_index_program_bottom(&programs);

        get_balanced_tower_combined_weight_or_difference(&mut programs, index_program_bottom)
            .err()
            .unwrap()
    }

    fn get_balanced_tower_combined_weight_or_difference(
        mut programs: &mut Vec<Program>,
        index: usize,
    ) -> Result<usize, isize> {
        let mut weights_towers_above = Vec::new();

        for index_program_above in programs[index].programs_above.to_vec() {
            match get_balanced_tower_combined_weight_or_difference(
                &mut programs,
                index_program_above,
            ) {
                Ok(weight_tower_above) => weights_towers_above.push((
                    programs[index_program_above].weight.unwrap(),
                    weight_tower_above,
                )),
                Err(error) => return Err(error),
            }
        }

        let mut frequencies = HashMap::new();

        for (_, weight_tower_above) in weights_towers_above.iter() {
            frequencies
                .entry(weight_tower_above)
                .and_modify(|weight| *weight += 1)
                .or_insert(1isize);
        }

        if frequencies.len() > 1 {
            let odd_weight = **frequencies
                .iter()
                .find(|&(_, frequency)| *frequency == 1)
                .unwrap()
                .0 as usize;

            let others_weight = **frequencies
                .iter()
                .find(|&(_, frequency)| *frequency != 1)
                .unwrap()
                .0 as usize;

            let weight_odd_one = weights_towers_above
                .iter()
                .find(|&(_, tower_weight)| *tower_weight == odd_weight)
                .unwrap()
                .0 as isize;

            let correct_weight = if odd_weight > others_weight {
                weight_odd_one as isize - (odd_weight as isize - others_weight as isize)
            } else {
                weight_odd_one as isize + (others_weight as isize - odd_weight as isize)
            };

            return Err(correct_weight);
        }

        programs[index].weight_tower_above = Some(
            weights_towers_above
                .iter()
                .map(|(_, weight_tower_above)| weight_tower_above)
                .sum::<usize>(),
        );

        Ok(programs[index].weight.unwrap() + programs[index].weight_tower_above.unwrap())
    }

    fn get_index_program_bottom(programs: &[Program]) -> usize {
        programs
            .iter()
            .enumerate()
            .find(|(index, _)| {
                !programs
                    .iter()
                    .any(|other_program| other_program.programs_above.contains(index))
            })
            .unwrap()
            .0
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(solve(input), 60);
    }
}
