use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Instruction {
    operation_operand_a: String,
    operation: Operation,
    operation_operand_b: isize,
    condition_operand_a: String,
    condition: Condition,
    condition_operand_b: isize,
}

#[derive(Debug)]
enum Operation {
    Decrease,
    Increase,
}

#[derive(Debug)]
enum Condition {
    Different,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new(
        r"^(?P<operation_operand_a>[a-z]+) (?P<operation>dec|inc) (?P<operation_operand_b>-?\d+) if (?P<condition_operand_a>[a-z]+) (?P<condition>[<>=!]=?) (?P<condition_operand_b>-?\d+)$",
    )
    .unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(&line).unwrap();

            let operation = match captures.name("operation").unwrap().as_str() {
                "dec" => Operation::Decrease,
                "inc" => Operation::Increase,
                _ => panic!(),
            };

            let condition = match captures.name("condition").unwrap().as_str() {
                "!=" => Condition::Different,
                "==" => Condition::Equal,
                ">" => Condition::Greater,
                ">=" => Condition::GreaterOrEqual,
                "<" => Condition::Less,
                "<=" => Condition::LessOrEqual,
                _ => panic!(),
            };

            Instruction {
                operation_operand_a: captures
                    .name("operation_operand_a")
                    .unwrap()
                    .as_str()
                    .to_string(),
                operation,
                operation_operand_b: captures
                    .name("operation_operand_b")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                condition_operand_a: captures
                    .name("condition_operand_a")
                    .unwrap()
                    .as_str()
                    .to_string(),
                condition,
                condition_operand_b: captures
                    .name("condition_operand_b")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            }
        })
        .collect()
}

mod part_1 {
    use crate::day_8::{decode_input, Condition, Operation};
    use std::collections::HashMap;

    pub fn solve(input: &str) -> isize {
        let mut registers = HashMap::new();

        decode_input(&input).into_iter().for_each(|instruction| {
            let value_condition_operand_a = *registers
                .entry(instruction.condition_operand_a)
                .or_insert(0);

            if match instruction.condition {
                Condition::Different => {
                    value_condition_operand_a != instruction.condition_operand_b
                }
                Condition::Equal => value_condition_operand_a == instruction.condition_operand_b,
                Condition::Greater => value_condition_operand_a > instruction.condition_operand_b,
                Condition::GreaterOrEqual => {
                    value_condition_operand_a >= instruction.condition_operand_b
                }
                Condition::Less => value_condition_operand_a < instruction.condition_operand_b,
                Condition::LessOrEqual => {
                    value_condition_operand_a <= instruction.condition_operand_b
                }
            } {
                let value_operation_operand_a = registers
                    .entry(instruction.operation_operand_a)
                    .or_insert(0);

                match instruction.operation {
                    Operation::Decrease => {
                        *value_operation_operand_a -= instruction.operation_operand_b
                    }
                    Operation::Increase => {
                        *value_operation_operand_a += instruction.operation_operand_b
                    }
                }
            }
        });

        *registers.values().max().unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(solve(input), 1);
    }
}

mod part_2 {
    use crate::day_8::{decode_input, Condition, Operation};
    use std::collections::HashMap;

    pub fn solve(input: &str) -> isize {
        let mut registers = HashMap::new();
        let mut highest_value_ever_held = None;

        decode_input(&input).into_iter().for_each(|instruction| {
            let value_condition_operand_a = *registers
                .entry(instruction.condition_operand_a)
                .or_insert(0);

            if match instruction.condition {
                Condition::Different => {
                    value_condition_operand_a != instruction.condition_operand_b
                }
                Condition::Equal => value_condition_operand_a == instruction.condition_operand_b,
                Condition::Greater => value_condition_operand_a > instruction.condition_operand_b,
                Condition::GreaterOrEqual => {
                    value_condition_operand_a >= instruction.condition_operand_b
                }
                Condition::Less => value_condition_operand_a < instruction.condition_operand_b,
                Condition::LessOrEqual => {
                    value_condition_operand_a <= instruction.condition_operand_b
                }
            } {
                let value_operation_operand_a = registers
                    .entry(instruction.operation_operand_a)
                    .or_insert(0);

                match instruction.operation {
                    Operation::Decrease => {
                        *value_operation_operand_a -= instruction.operation_operand_b
                    }
                    Operation::Increase => {
                        *value_operation_operand_a += instruction.operation_operand_b
                    }
                }

                let largest_value = *registers.values().max().unwrap();

                if highest_value_ever_held.is_none()
                    || largest_value > highest_value_ever_held.unwrap()
                {
                    highest_value_ever_held = Some(largest_value);
                }
            }
        });

        highest_value_ever_held.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(solve(input), 10);
    }
}
