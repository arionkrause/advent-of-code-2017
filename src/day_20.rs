use super::print_day;
use regex::Regex;

pub fn solve(input: &str) {
    print_day!(file!());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

#[derive(Debug)]
struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl Particle {
    fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
struct Acceleration {
    x: isize,
    y: isize,
    z: isize,
}

fn decode_input(input: &str) -> Vec<Particle> {
    let re =
        Regex::new(r"^p=<(?P<position_x>-?\d+),(?P<position_y>-?\d+),(?P<position_z>-?\d+)>, v=<(?P<velocity_x>-?\d+),(?P<velocity_y>-?\d+),(?P<velocity_z>-?\d+)>, a=<(?P<acceleration_x>-?\d+),(?P<acceleration_y>-?\d+),(?P<acceleration_z>-?\d+)>$").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            Particle {
                position: Position {
                    x: captures
                        .name("position_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    y: captures
                        .name("position_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    z: captures
                        .name("position_z")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                },
                velocity: Velocity {
                    x: captures
                        .name("velocity_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    y: captures
                        .name("velocity_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    z: captures
                        .name("velocity_z")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                },
                acceleration: Acceleration {
                    x: captures
                        .name("acceleration_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    y: captures
                        .name("acceleration_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    z: captures
                        .name("acceleration_z")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                },
            }
        })
        .collect()
}

mod part_1 {
    use crate::day_20;
    use crate::day_20::decode_input;

    pub fn solve(input: &str) -> usize {
        let mut particles = decode_input(&input);

        for _ in 0..1000 {
            particles.iter_mut().for_each(day_20::Particle::update);
        }

        particles
            .iter()
            .enumerate()
            .min_by(|(_, particle_a), (_, particle_b)| {
                particle_a
                    .get_manhattan_distance()
                    .cmp(&particle_b.get_manhattan_distance())
            })
            .unwrap()
            .0
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

        assert_eq!(solve(input), 0);
    }
}
