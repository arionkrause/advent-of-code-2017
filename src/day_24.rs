use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Component {
    port_a: usize,
    port_b: usize,
}

#[derive(Clone, Debug)]
struct State {
    open_port: usize,
    used_component_indices: Vec<usize>,
}

fn decode_input(input: &str) -> Vec<Component> {
    let re = Regex::new(r"^(?P<port_a>\d+)/(?P<port_b>\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Component {
                port_a: captures.name("port_a").unwrap().as_str().parse().unwrap(),
                port_b: captures.name("port_b").unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn get_strength(components: &[Component], state: &State) -> usize {
    state
        .used_component_indices
        .iter()
        .map(|&index| components[index].port_a + components[index].port_b)
        .sum()
}

mod part_1 {
    use crate::day_24::{decode_input, get_strength, State};
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let mut components = decode_input(&input);

        components.sort_by(|component_a, component_b| {
            component_a
                .port_a
                .max(component_a.port_b)
                .cmp(&component_b.port_a.max(component_b.port_b))
                .then(
                    component_a
                        .port_a
                        .max(component_a.port_b)
                        .cmp(&component_b.port_a.max(component_b.port_b)),
                )
        });

        let state = State {
            open_port: 0,
            used_component_indices: vec![],
        };

        let mut queue = VecDeque::new();
        queue.push_front(state);
        let mut strongest_bridge = None;

        while let Some(state) = queue.pop_front() {
            let strength = get_strength(&components, &state);

            if strongest_bridge.is_none() || strength > strongest_bridge.unwrap() {
                strongest_bridge = Some(strength);
            }

            for (index, component) in components.iter().enumerate() {
                if state.used_component_indices.contains(&index) {
                    continue;
                }

                if component.port_a != state.open_port && component.port_b != state.open_port {
                    continue;
                }

                let mut state_clone = state.clone();
                state_clone.used_component_indices.push(index);

                state_clone.open_port = if component.port_a == state_clone.open_port {
                    component.port_b
                } else {
                    component.port_a
                };

                queue.push_back(state_clone);
            }
        }

        strongest_bridge.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(solve(input), 31);
    }
}

mod part_2 {
    use crate::day_24::{decode_input, get_strength, State};
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let mut components = decode_input(&input);

        components.sort_by(|component_a, component_b| {
            component_a
                .port_a
                .max(component_a.port_b)
                .cmp(&component_b.port_a.max(component_b.port_b))
                .then(
                    component_a
                        .port_a
                        .max(component_a.port_b)
                        .cmp(&component_b.port_a.max(component_b.port_b)),
                )
        });

        let state = State {
            open_port: 0,
            used_component_indices: vec![],
        };

        let mut queue = VecDeque::new();
        queue.push_front(state);
        let mut longest_bridge = None;
        let mut strongest_longest_bridge = None;

        while let Some(state) = queue.pop_front() {
            let length = state.used_component_indices.len();

            if longest_bridge.is_none() || length >= longest_bridge.unwrap() {
                let strength = get_strength(&components, &state);

                if longest_bridge.is_none() || length > longest_bridge.unwrap() {
                    longest_bridge = Some(length);
                    strongest_longest_bridge = Some(strength);
                } else if strongest_longest_bridge.is_none()
                    || strength > strongest_longest_bridge.unwrap()
                {
                    strongest_longest_bridge = Some(strength);
                }
            }

            for (index, component) in components.iter().enumerate() {
                if state.used_component_indices.contains(&index) {
                    continue;
                }

                if component.port_a != state.open_port && component.port_b != state.open_port {
                    continue;
                }

                let mut state_clone = state.clone();
                state_clone.used_component_indices.push(index);

                state_clone.open_port = if component.port_a == state_clone.open_port {
                    component.port_b
                } else {
                    component.port_a
                };

                queue.push_back(state_clone);
            }
        }

        strongest_longest_bridge.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(solve(input), 19);
    }
}
