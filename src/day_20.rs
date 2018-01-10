const DAY_20_INPUT: &'static str = include_str!("../data/day_20_input");
const ITERATIONS: usize = 1000;

use std::error::Error;
use std::collections::HashMap;
use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

pub fn run() -> Result<(), Box<Error>> {
    println!("*** Day 20: Particle Swarm ***");
    println!("Input: {}", DAY_20_INPUT);
    println!("solution 1: {:?}", solution_1(DAY_20_INPUT)?);
    println!("solution 2: {:?}", solution_2(DAY_20_INPUT)?);
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
struct Particle {
    p: Position,
    v: Velocity,
    a: Acceleration,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
struct Acceleration {
    x: isize,
    y: isize,
    z: isize,
}

fn solution_1(s: &str) -> Result<Option<usize>, Errors<PointerOffset, char, &str>> {
    let (mut particles, _) = Particle::parse_many(s)?;
    simulate(&mut particles, ITERATIONS);
    let closest_idx_particle = particles.iter().enumerate().fold(
        None,
        |acc, (idx, next)| match acc {
            Some((_, prev_manhattan)) => {
                let next_manhattan = manhattan_dist(next.p);
                if next_manhattan < prev_manhattan {
                    Some((idx, next_manhattan))
                } else {
                    acc
                }
            }
            None => Some((idx, manhattan_dist(next.p))),
        },
    );
    Ok(closest_idx_particle.map(|(idx, _)| idx))
}

fn solution_2(s: &str) -> Result<usize, Errors<PointerOffset, char, &str>> {
    let (particles, _) = Particle::parse_many(s)?;
    let mut particles_map = particles
        .into_iter()
        .enumerate()
        .map(|(idx, p)| (idx, p))
        .collect();
    simulate_with_collisions(&mut particles_map, ITERATIONS);
    Ok(particles_map.len())
}

fn simulate(v: &mut Vec<Particle>, times: usize) -> () {
    for _ in 0..times {
        for p in v.iter_mut() {
            p.update()
        }
    }
}

fn simulate_with_collisions(v: &mut HashMap<usize, Particle>, times: usize) -> () {
    let mut collision_checker = HashMap::new();
    for _ in 0..times {
        for (idx, p) in v.iter_mut() {
            p.update();
            collision_checker.entry(p.p).or_insert(vec![]).push(*idx);
        }
        for (_, indices) in collision_checker.drain() {
            if indices.len() > 1 {
                for idx in indices {
                    v.remove(&idx);
                }
            }
        }
    }
}

fn manhattan_dist(p: Position) -> usize {
    (p.x.abs() + p.y.abs() + p.z.abs()) as usize
}

impl Particle {
    fn update(&mut self) {
        self.v.x += self.a.x;
        self.v.y += self.a.y;
        self.v.z += self.a.z;
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.p.z += self.v.z;
    }
}

macro_rules! build_parser {
    ($builder: ident ~ $c: tt) => {
        string(concat!($c, "=<"))
            .with(
                tabs_or_spaces!()
                .with(
                    number_parser!(isize)
                        .skip(tabs_or_spaces!().with(char(',')))
                    .and(
                        number_parser!(isize)
                    ).skip(tabs_or_spaces!().with(char(',')))
                    .and(
                        number_parser!(isize)
                    ).map (|((x, y), z)| $builder { x, y, z } )
                )
            )
        .skip(
           tabs_or_spaces!().with(char('>'))
        )
    };
}

macro_rules! particle_parser {
    () => {
        build_parser!(Position ~ "p")
            .skip(
                tabs_or_spaces!().with(char(','))
            ).and(
                tabs_or_spaces!().with(
        build_parser!(Velocity ~ "v"))
                )
            .skip(
                tabs_or_spaces!().with(char(','))
            ).and(
            tabs_or_spaces!().with(
        build_parser!(Acceleration ~ "a")))
        .map( |((p, v), a)| Particle { p, v, a } )
    };
}

impl Particle {
    fn parse_many(s: &str) -> Result<(Vec<Particle>, &str), Errors<PointerOffset, char, &str>> {
        let mut parser = spaces().with(sep_by(particle_parser!(), spaces()));
        parser.easy_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use day_20::*;

    #[test]
    fn build_parser_test() {
        let mut parser = build_parser!(Position ~ "p");
        let (p, _) = parser.easy_parse("p=<-833,-499,-1391>").unwrap();
        assert_eq!(
            p,
            Position {
                x: -833,
                y: -499,
                z: -1391,
            }
        );
    }

    #[test]
    fn particles_parser_test() {
        let (particles, _) = Particle::parse_many(DAY_20_INPUT).unwrap();
        assert_eq!(particles.len(), 1000);
    }

    #[test]
    fn solution_1_test() {
        let r = solution_1(DAY_20_INPUT).unwrap().unwrap();
        assert_eq!(r, 457);
    }

    #[test]
    fn solution_2_test() {
        let r = solution_2(DAY_20_INPUT).unwrap();
        assert_eq!(r, 448);
    }
}
