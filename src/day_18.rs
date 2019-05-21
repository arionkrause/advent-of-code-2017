use super::print_day;
use crate::day_18::Runnable::{
    JgzRegisterRegister, JgzRegisterValue, JgzValueRegister, JgzValueValue,
};
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
enum Runnable {
    AddRegister { operand_a: char, operand_b: char },
    AddValue { operand_a: char, operand_b: isize },
    JgzRegisterRegister { operand_a: char, operand_b: char },
    JgzRegisterValue { operand_a: char, operand_b: isize },
    JgzValueRegister { operand_a: isize, operand_b: char },
    JgzValueValue { operand_a: isize, operand_b: isize },
    ModRegister { operand_a: char, operand_b: char },
    ModValue { operand_a: char, operand_b: isize },
    MulRegister { operand_a: char, operand_b: char },
    MulValue { operand_a: char, operand_b: isize },
    Rcv { operand_a: char },
    SetRegister { operand_a: char, operand_b: char },
    SetValue { operand_a: char, operand_b: isize },
    Snd { operand_a: char },
}

fn decode_input(input: &str) -> Vec<Runnable> {
    let re_add = Regex::new(r"^add (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_jgz =
        Regex::new(r"^jgz (?P<operand_a>[a-z]|-?\d+) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_mod = Regex::new(r"^mod (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_mul = Regex::new(r"^mul (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_rcv = Regex::new(r"^rcv (?P<operand_a>[a-z])$").unwrap();
    let re_set = Regex::new(r"^set (?P<operand_a>[a-z]) (?P<operand_b>[a-z]|-?\d+)$").unwrap();
    let re_snd = Regex::new(r"^snd (?P<operand_a>[a-z])$").unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(captures) = re_add.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let register_or_value = captures.name("operand_b").unwrap().as_str();

                match register_or_value.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::AddValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::AddRegister {
                            operand_a,
                            operand_b: register_or_value.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_jgz.captures(line) {
                let operand_a = captures.name("operand_a").unwrap().as_str();
                let operand_b = captures.name("operand_b").unwrap().as_str();
                let is_operand_a_register = operand_a.parse::<isize>().is_err();
                let is_operand_b_register = operand_b.parse::<isize>().is_err();

                if is_operand_a_register {
                    if is_operand_b_register {
                        return JgzRegisterRegister {
                            operand_a: operand_a.chars().next().unwrap(),
                            operand_b: operand_b.chars().next().unwrap(),
                        };
                    }

                    return JgzRegisterValue {
                        operand_a: operand_a.chars().next().unwrap(),
                        operand_b: operand_b.parse().unwrap(),
                    };
                }

                if is_operand_b_register {
                    return JgzValueRegister {
                        operand_a: operand_a.parse().unwrap(),
                        operand_b: operand_b.chars().next().unwrap(),
                    };
                }

                return JgzValueValue {
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

                let register_or_value = captures.name("operand_b").unwrap().as_str();

                match register_or_value.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::ModValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::ModRegister {
                            operand_a,
                            operand_b: register_or_value.chars().next().unwrap(),
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

                let register_or_value = captures.name("operand_b").unwrap().as_str();

                match register_or_value.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::MulValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::MulRegister {
                            operand_a,
                            operand_b: register_or_value.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_rcv.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                return Runnable::Rcv { operand_a };
            }

            if let Some(captures) = re_set.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                let register_or_value = captures.name("operand_b").unwrap().as_str();

                match register_or_value.parse::<isize>() {
                    Ok(value) => {
                        return Runnable::SetValue {
                            operand_a,
                            operand_b: value,
                        }
                    }
                    Err(_) => {
                        return Runnable::SetRegister {
                            operand_a,
                            operand_b: register_or_value.chars().next().unwrap(),
                        }
                    }
                };
            }

            if let Some(captures) = re_snd.captures(line) {
                let operand_a = captures
                    .name("operand_a")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap();

                return Runnable::Snd { operand_a };
            }

            panic!();
        })
        .collect()
}

mod part_1 {
    use crate::day_18::{decode_input, Runnable};
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut registers: HashMap<char, isize> = HashMap::new();
        let instructions = decode_input(&input);
        let mut program_counter = 0;
        let mut frequency_last_sound_played = None;

        loop {
            let instruction = &instructions[program_counter as usize];

            match instruction {
                Runnable::AddRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value += value_b)
                        .or_insert(value_b);

                    program_counter += 1;
                }
                Runnable::AddValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value += operand_b)
                        .or_insert(*operand_b);

                    program_counter += 1;
                }
                Runnable::JgzRegisterRegister {
                    operand_a,
                    operand_b,
                } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();
                    let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();

                    if value_a > 0 {
                        program_counter += value_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JgzRegisterValue {
                    operand_a,
                    operand_b,
                } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();

                    if value_a > 0 {
                        program_counter += operand_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JgzValueRegister {
                    operand_a,
                    operand_b,
                } => {
                    if *operand_a > 0 {
                        let value_b = *registers.get(&operand_b).or(Some(&0)).unwrap();
                        program_counter += value_b;
                    } else {
                        program_counter += 1;
                    }
                }
                Runnable::JgzValueValue {
                    operand_a,
                    operand_b,
                } => {
                    if *operand_a > 0 {
                        program_counter += operand_b;
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
                        .and_modify(|value| *value %= operand_b)
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
                }
                Runnable::MulValue {
                    operand_a,
                    operand_b,
                } => {
                    registers
                        .entry(*operand_a)
                        .and_modify(|value| *value *= operand_b)
                        .or_insert(0);

                    program_counter += 1;
                }
                Runnable::Rcv { operand_a } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();

                    if value_a > 0 {
                        break;
                    }

                    program_counter += 1;
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
                Runnable::Snd { operand_a } => {
                    let value_a = *registers.get(&operand_a).or(Some(&0)).unwrap();
                    frequency_last_sound_played = Some(value_a as usize);
                    program_counter += 1;
                }
            }
        }

        frequency_last_sound_played.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

        assert_eq!(solve(input), 4);
    }
}
