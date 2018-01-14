use std::collections::HashMap;
use std::error;

use self::UncleanState::*;
use self::FacingDirection::*;
use self::TurnDirection::*;
use self::Action::*;

use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;
use combine::easy;

const DAY_22_INPUT: &'static str = include_str!("../data/day_22_input");

pub fn run() -> Result<(), Box<error::Error>> {
    println!("*** Day 22: Sporifica Virus ***");
    println!("Input: {}", DAY_22_INPUT);
    let virus_state = VirusState::parse(DAY_22_INPUT)?;
    println!("Solution 1: {}", solution_1(&virus_state, 10000));
    println!("Solution 2: {}", solution_2(&virus_state, 10000000));
    Ok(())
}

fn solution_1(state: &VirusState, iterations: usize) -> usize {
    let it = state.to_burst_activity_iter_1();
    it.take(iterations)
        .filter(|a| a.action_taken == Infect)
        .count()
}

fn solution_2(state: &VirusState, iterations: usize) -> usize {
    let it = state.to_burst_activity_iter_2();
    it.take(iterations)
        .filter(|a| a.action_taken == Infect)
        .count()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum UncleanState {
    Weakened,
    Flagged,
    Infected,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Action {
    Infect,
    Clean,
    Weaken,
    Flag,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TurnDirection {
    Left,
    Right,
    Stay,
    Reverse,
}

impl TurnDirection {
    fn turn_from(&self, currently_facing: &FacingDirection) -> FacingDirection {
        match self {
            &Left => match currently_facing {
                &North => West,
                &East => North,
                &South => East,
                &West => South,
            },
            &Right => match currently_facing {
                &North => East,
                &East => South,
                &South => West,
                &West => North,
            },
            &Reverse => match currently_facing {
                &North => South,
                &East => West,
                &South => North,
                &West => East,
            },
            &Stay => *currently_facing,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum FacingDirection {
    North,
    East,
    South,
    West,
}

impl FacingDirection {
    fn move_from(&self, coord: &Coord) -> Coord {
        match self {
            &North => Coord {
                y: coord.y + 1,
                ..*coord
            },
            &East => Coord {
                x: coord.x + 1,
                ..*coord
            },
            &South => Coord {
                y: coord.y - 1,
                ..*coord
            },
            &West => Coord {
                x: coord.x - 1,
                ..*coord
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct VirusState {
    position: Coord,
    facing: FacingDirection,
    unclean_nodes: HashMap<Coord, UncleanState>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct BurstActivity {
    pos_start: Coord,
    pos_end: Coord,
    facing_start: FacingDirection,
    facing_end: FacingDirection,
    direction_turned: TurnDirection,
    action_taken: Action,
}

#[derive(Debug)]
struct BurstActivityIteratorPart1 {
    state: VirusState,
}

impl Iterator for BurstActivityIteratorPart1 {
    type Item = BurstActivity;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pos_infected = self.state.unclean_nodes.contains_key(&self.state.position);
        let action_taken = if current_pos_infected { Clean } else { Infect };
        let direction_turned = if current_pos_infected { Right } else { Left };
        let facing_start = self.state.facing;
        let facing_end = direction_turned.turn_from(&self.state.facing);
        let pos_start = self.state.position;
        let pos_end = facing_end.move_from(&self.state.position);
        // <-- update state start
        match action_taken {
            Clean => {
                self.state.unclean_nodes.remove(&self.state.position);
            }
            Infect => {
                self.state
                    .unclean_nodes
                    .insert(self.state.position, Infected);
            }
            _ => (),
        }
        self.state.facing = facing_end;
        self.state.position = pos_end;
        //      update state end -->
        Some(BurstActivity {
            pos_start,
            pos_end,
            facing_start,
            facing_end,
            direction_turned,
            action_taken,
        })
    }
}

#[derive(Debug)]
struct BurstActivityIteratorPart2 {
    state: VirusState,
}

impl Iterator for BurstActivityIteratorPart2 {
    type Item = BurstActivity;

    fn next(&mut self) -> Option<Self::Item> {
        let (action_taken, direction_turned) = {
            let current_pos_status = self.state.unclean_nodes.get(&self.state.position);
            let action = match current_pos_status {
                None => Weaken,
                Some(&Weakened) => Infect,
                Some(&Infected) => Flag,
                Some(&Flagged) => Clean,
            };
            let direction = match current_pos_status {
                None => Left,
                Some(&Weakened) => Stay,
                Some(&Infected) => Right,
                Some(&Flagged) => Reverse,
            };
            (action, direction)
        };
        let facing_start = self.state.facing;
        let facing_end = direction_turned.turn_from(&self.state.facing);
        let pos_start = self.state.position;
        let pos_end = facing_end.move_from(&self.state.position);
        // <-- update state start
        match action_taken {
            Clean => {
                self.state.unclean_nodes.remove(&self.state.position);
            }
            Weaken => {
                self.state
                    .unclean_nodes
                    .insert(self.state.position, Weakened);
            }
            Infect => {
                self.state
                    .unclean_nodes
                    .insert(self.state.position, Infected);
            }
            Flag => {
                self.state
                    .unclean_nodes
                    .insert(self.state.position, Flagged);
            }
        }
        self.state.facing = facing_end;
        self.state.position = pos_end;
        //      update state end -->
        Some(BurstActivity {
            pos_start,
            pos_end,
            facing_start,
            facing_end,
            direction_turned,
            action_taken,
        })
    }
}

impl VirusState {
    /// Takes a Matrix where true means infected, false means clean and returns a State
    fn from(data: &Vec<Vec<bool>>) -> Result<VirusState, &'static str> {
        let height = data.len();
        let width = data.iter().map(|row| row.len()).next().unwrap_or(0);
        if height % 2 == 0 || width % 2 == 0 {
            Err("Cannot work with matrices with even height or width")
        } else if !data.iter().map(|row| row.len()).all(|cols| cols == width) {
            Err("Not all rows have the same width")
        } else {
            let x_zero_col = width / 2;
            let y_zero_row = height / 2;
            let mut unclean_nodes = HashMap::with_capacity(height * width);
            for i in 0..height {
                let current_y = y_zero_row as isize - i as isize;
                for j in 0..width {
                    let current_x = j as isize - x_zero_col as isize;
                    if let Some(&true) = data.get(i).and_then(|row| row.get(j)) {
                        unclean_nodes.insert(
                            Coord {
                                x: current_x,
                                y: current_y,
                            },
                            Infected,
                        );
                    }
                }
            }
            Ok(VirusState {
                position: Coord { x: 0, y: 0 },
                facing: North,
                unclean_nodes: unclean_nodes,
            })
        }
    }

    fn parse(s: &str) -> Result<VirusState, Errors<PointerOffset, char, &str>> {
        let mut parser = sep_by(
            many1(try(char('.').map(|_| false)).or(char('#').map(|_| true))),
            spaces(),
        ).and_then(|matrix| {
            VirusState::from(&matrix).map_err(|e| Error::Message(easy::Info::Borrowed(e)))
        });
        let (s, _) = parser.easy_parse(s)?;
        Ok(s)
    }

    fn to_burst_activity_iter_1(&self) -> BurstActivityIteratorPart1 {
        BurstActivityIteratorPart1 {
            state: self.clone(),
        }
    }
    fn to_burst_activity_iter_2(&self) -> BurstActivityIteratorPart2 {
        BurstActivityIteratorPart2 {
            state: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use day_22::*;

    const TEST_INPUT: &'static str = include_str!("../data/day_22_test_input");

    #[test]
    fn state_new_test() {
        let s = VirusState::from(&vec![
            vec![false, false, true],
            vec![true, false, false],
            vec![false, true, true],
        ]).unwrap();
        let expected_infected = hashmap![
            Coord { x: 1, y: 1 } => Infected,
            Coord { x: -1, y: 0 } => Infected,
            Coord { x: 1, y: -1 } => Infected,
            Coord { x: 0, y: -1 } => Infected,
        ];
        assert_eq!(s.unclean_nodes, expected_infected);
        assert_eq!(s.position, Coord { x: 0, y: 0 });
    }

    #[test]
    fn burst_activity_iterator_test() {
        let s = VirusState::from(&vec![
            vec![false, false, true],
            vec![true, false, false],
            vec![false, true, true],
        ]).unwrap();
        let iter = s.to_burst_activity_iter_1();
        for s in iter.take(1000) {
            // something trivial
            assert!(s.action_taken == Infect || s.action_taken == Clean)
        }
    }

    #[test]
    fn state_parser_test() {
        let s = VirusState::parse(TEST_INPUT).unwrap();
        let expected_infected = hashmap![
            Coord { x: -1, y: 0 } => Infected,
            Coord { x: 1, y: 1 } => Infected,
        ];
        assert_eq!(s.unclean_nodes, expected_infected);
    }

    #[test]
    fn solution_1_test_input_test() {
        let s = VirusState::parse(TEST_INPUT).unwrap();
        assert_eq!(solution_1(&s, 7), 5);
        assert_eq!(solution_1(&s, 70), 41);
        assert_eq!(solution_1(&s, 10000), 5587);
    }

    #[test]
    fn solution_1_real_input_test() {
        let s = VirusState::parse(DAY_22_INPUT).unwrap();
        assert_eq!(solution_1(&s, 10000), 5256);
    }

    #[test]
    fn solution_2_real_input_test() {
        let s = VirusState::parse(DAY_22_INPUT).unwrap();
        assert_eq!(solution_2(&s, 10000000), 2511345);
    }
}
