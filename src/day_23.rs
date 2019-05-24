use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Clone, Debug)]
enum Runnable {
    JnzRegisterRegister { operand_a: char, operand_b: char },
    JnzRegisterValue { operand_a: char, operand_b: isize },
    JnzValueRegister { operand_a: isize, operand_b: char },
    JnzValueValue { operand_a: isize, operand_b: isize },
    ModRegister { operand_a: char, operand_b: char },
    ModValue { operand_a: char, operand_b: isize },
    MulRegister { operand_a: char, operand_b: char },
    MulValue { operand_a: char, operand_b: isize },
    SetRegister { operand_a: char, operand_b: char },
    SetValue { operand_a: char, operand_b: isize },
    SubRegister { operand_a: char, operand_b: char },
    SubValue { operand_a: char, operand_b: isize },
}

fn decode_input(input: &str) -> Vec<Runnable> {
    let re_jnz =
        Regex::new(r"^jnz (?P<operand_a>[a-z]|-?\d+) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_mod = Regex::new(r"^mod (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_mul = Regex::new(r"^mul (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_set = Regex::new(r"^set (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_sub = Regex::new(r"^sub (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(captures) = re_jnz.captures(line) {
                let operand_a = captures.name("operand_a").unwrap().as_str();
                let operand_b = captures.name("operand_b").unwrap().as_str();
                let is_operand_a_register = operand_a.parse::<isize>().is_err();
                let is_operand_b_register = operand_b.parse::<isize>().is_err();

                if is_operand_a_register {
                    if is_operand_b_register {
                        return Runnable::JnzRegisterRegister {
                            operand_a: operand_a.chars().next().unwrap(),
                            operand_b: operand_b.chars().next().unwrap(),
                        };
                    }

                    return Runnable::JnzRegisterValue {
                        operand_a: operand_a.chars().next().unwrap(),
                        operand_b: operand_b.parse().unwrap(),
                    };
                }

                if is_operand_b_register {
                    return Runnable::JnzValueRegister {
                        operand_a: operand_a.parse().unwrap(),
                        operand_b: operand_b.chars().next().unwrap(),
                    };
                }

                return Runnable::JnzValueValue {
                    operand_a: operand_a.parse().unwrap(),
                    operand_b: operand_b.parse().unwrap(),
                };
            }

            if let Some(captures) = re_mod.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let operand_b = captures.name("operand_b").unwrap().as_str();

                match operand_b.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::ModValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::ModRegister {
                            operand_a,
                            operand_b: operand_b.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_mul.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let operand_b = captures.name("operand_b").unwrap().as_str();

                match operand_b.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::MulValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::MulRegister {
                            operand_a,
                            operand_b: operand_b.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_set.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let operand_b = captures.name("operand_b").unwrap().as_str();

                match operand_b.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::SetValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::SetRegister {
                            operand_a,
                            operand_b: operand_b.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_sub.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let operand_b = captures.name("operand_b").unwrap().as_str();

                match operand_b.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::SubValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::SubRegister {
                            operand_a,
                            operand_b: operand_b.chars().next().unwrap(),
                        }
                    }
                };
            }

            panic!();
        })
        .collect()
}

mod part_1 {
    use crate::day_23::{decode_input, Runnable};
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut registers: HashMap<char, isize> = HashMap::new();
        let instructions = decode_input(&input);
        let mut program_counter = 0;
        let mut amount_times_mul_invoked = 0;

        loop {
            let instruction = &instructions[program_counter as usize];

            match instruction {
                Runnable::JnzRegisterRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    if value_a != 0 {
                        program_counter += value_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JnzRegisterValue {
                    operand_a,
                    operand_b,
                } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();

                    if value_a != 0 {
                        program_counter += *operand_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JnzValueRegister {
                    operand_a,
                    operand_b,
                } => {
                    if *operand_a != 0 {
                        let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();
                        program_counter += value_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JnzValueValue {
                    operand_a,
                    operand_b,
                } => {
                    if *operand_a != 0 {
                        program_counter += *operand_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::ModRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value %= value_b)
                        .or_insert(0);

                    program_counter += 1;
                }
                Runnable::ModValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value %= *operand_b)
                        .or_insert(0);

                    program_counter += 1;
                }
                Runnable::MulRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value *= value_b)
                        .or_insert(0);

                    program_counter += 1;
                    amount_times_mul_invoked += 1;
                }
                Runnable::MulValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value *= *operand_b)
                        .or_insert(0);

                    program_counter += 1;
                    amount_times_mul_invoked += 1;
                }
                Runnable::SetRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value = value_b)
                        .or_insert(value_b);

                    program_counter += 1;
                }
                Runnable::SetValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value = *operand_b)
                        .or_insert(*operand_b);

                    program_counter += 1;
                }
                Runnable::SubRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value -= value_b)
                        .or_insert(value_b);

                    program_counter += 1;
                }
                Runnable::SubValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value -= *operand_b)
                        .or_insert(*operand_b);

                    program_counter += 1;
                }
            }

            if program_counter >= instructions.len() as isize {
                break;
            }
        }

        amount_times_mul_invoked
    }
}
