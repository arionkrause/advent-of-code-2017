use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

#[derive(Debug)]
struct Layer {
    depth: usize,
    range: usize,
}

fn decode_input(input: &str) -> Vec<Layer> {
    let re = Regex::new(r"^(?P<depth>\d+): (?P<range>\d+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(&line).unwrap();

            Layer {
                depth: captures.name("depth").unwrap().as_str().parse().unwrap(),
                range: captures.name("range").unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

mod part_1 {
    use crate::day_13::decode_input;

    pub fn solve(input: &str) -> usize {
        decode_input(&input)
            .into_iter()
            .map(|layer| {
                if layer.depth % (layer.range * 2 - 2) == 0 {
                    layer.range * layer.depth
                } else {
                    0
                }
            })
            .sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(solve(&input), 24);
    }
}

mod part_2 {
    use crate::day_13::decode_input;

    pub fn solve(input: &str) -> usize {
        let layers = decode_input(&input);
        let mut delay = 10;

        loop {
            if layers
                .iter()
                .all(|layer| (layer.depth + delay) % (layer.range * 2 - 2) != 0)
            {
                return delay;
            }

            delay += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(solve(&input), 10);
    }
}
