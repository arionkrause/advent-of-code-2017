use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct State {
    currently_zero: Step,
    currently_one: Step,
}

#[derive(Debug)]
struct Step {
    new_value: usize,
    movement: isize,
    next_state_index: usize,
}

fn decode_input(input: &str) -> (Vec<State>, usize, usize) {
    (
        get_states(&input),
        get_initial_state_index(&input) as usize,
        get_amount_steps(&input),
    )
}

fn get_states(input: &str) -> Vec<State> {
    let re = Regex::new(
        r"In state [A-Z]:
 {2}If the current value is 0:
 {4}- Write the value (?P<zero_new_value>\d).
 {4}- Move one slot to the (?P<zero_movement>right|left).
 {4}- Continue with state (?P<zero_next_state_index>[A-Z]).
 {2}If the current value is 1:
 {4}- Write the value (?P<one_new_value>\d).
 {4}- Move one slot to the (?P<one_movement>right|left).
 {4}- Continue with state (?P<one_next_state_index>[A-Z]).",
    )
    .unwrap();

    let mut states = Vec::new();

    for captures in re.captures_iter(&input) {
        let zero_new_value = captures
            .name("zero_new_value")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let zero_movement = if captures.name("zero_movement").unwrap().as_str() == "right" {
            1
        } else {
            -1
        };

        let zero_next_state_index = captures
            .name("zero_next_state_index")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap() as usize
            - 65;

        let currently_zero = Step {
            new_value: zero_new_value,
            movement: zero_movement,
            next_state_index: zero_next_state_index as usize,
        };

        let one_new_value = captures
            .name("one_new_value")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let one_movement = if captures.name("one_movement").unwrap().as_str() == "right" {
            1
        } else {
            -1
        };

        let one_next_state_index = captures
            .name("one_next_state_index")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap() as usize
            - 65;

        let currently_one = Step {
            new_value: one_new_value,
            movement: one_movement,
            next_state_index: one_next_state_index as usize,
        };

        states.push(State {
            currently_zero,
            currently_one,
        })
    }

    states
}

fn get_initial_state_index(input: &str) -> usize {
    Regex::new(r"Begin in state (?P<id>[A-Z]).")
        .unwrap()
        .captures(&input)
        .unwrap()
        .name("id")
        .unwrap()
        .as_str()
        .chars()
        .next()
        .unwrap() as usize
        - 65
}

fn get_amount_steps(input: &str) -> usize {
    Regex::new(r"Perform a diagnostic checksum after (?P<amount_steps>\d+) steps.")
        .unwrap()
        .captures(&input)
        .unwrap()
        .name("amount_steps")
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

mod part_1 {
    use crate::day_25::decode_input;

    pub fn solve(input: &str) -> usize {
        let (states, initial_state_index, amount_steps) = decode_input(&input);
        let mut tape = vec![0; 4];
        let mut current_state_index = initial_state_index;
        let mut current_position = tape.len() / 2;

        for _ in 0..amount_steps {
            let state = &states[current_state_index];

            let step = if tape[current_position] == 0 {
                &state.currently_zero
            } else {
                &state.currently_one
            };

            tape[current_position] = step.new_value;
            current_position = (current_position as isize + step.movement) as usize;

            if current_position == 0 || current_position == tape.len() - 1 {
                let mut new_tape = vec![0; tape.len() * 2];

                for (index, value) in tape.iter().enumerate() {
                    new_tape[index + tape.len() / 2] = *value;
                }

                current_position += tape.len() / 2;
                tape = new_tape;
            }

            current_state_index = step.next_state_index;
        }

        tape.iter().filter(|value| **value == 1).count()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

        assert_eq!(solve(input), 3);
    }
}
