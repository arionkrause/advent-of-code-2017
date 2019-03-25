use super::print_day;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
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
