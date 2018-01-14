use std::fmt::Display;
use std::fmt;
use std::error;
use std::collections::{HashMap, HashSet};

use self::State::*;
use self::Rotation::*;

use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;
use combine::easy;

const DAY_21_MATRIX: &str = include_str!("../data/day_21_input_matrix");
const DAY_21_RULES: &str = include_str!("../data/day_21_input");
const MIN_COORD: Coord = Coord { i: 0, j: 0 };

pub fn run() -> Result<(), Box<error::Error>> {
    println!("*** Day 21: Fractal Art ***");
    println!("Input matrix: {}", DAY_21_MATRIX);
    println!("Rules: {}", DAY_21_RULES);
    let mut the_matrix = SquareMatrix::parse(DAY_21_MATRIX)?;
    let parsed_rules = TransformRule::parse_many(DAY_21_RULES)?;
    let mappings = TransformationMappings::from_rules(&parsed_rules)?;
    //    println!("the_matrix:\n{}", the_matrix);
    println!("Solution 1: {}", solution(&mut the_matrix, &mappings, 5)?);
    //    println!("the_matrix:\n{}", the_matrix);
    println!("Solution 2: {}", solution(&mut the_matrix, &mappings, 13)?);
    //    println!("the_matrix:\n{}", the_matrix);
    Ok(())
}

fn solution<'a, 'b>(
    matrix: &'a mut SquareMatrix,
    mappings: &'a TransformationMappings<'b>,
    iterations: usize,
) -> Result<usize, &'static str> {
    matrix.expand(mappings, iterations)?;
    let on_count = matrix._data.iter().filter(|s| **s == On).count();
    Ok(on_count)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum State {
    On,
    Off,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Rotation {
    R90,
    R180,
    R270,
    R360,
}

impl Display for SquareMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(self.length * (self.length + 1)); // to account for newlines
        for (idx, state) in self._data.iter().enumerate() {
            match state {
                &Off => s.push('.'),
                &On => s.push('#'),
            }
            if idx % (self.length - 1) == 0 {
                s.push('\n');
            }
        }
        f.write_str(s.as_str())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Coord {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SquareMatrix {
    _data: Vec<State>,
    length: usize,
    max_i: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct InternalIdx(usize);

#[derive(Debug, Eq, PartialEq)]
struct ExpansionData {
    plan: TileExpansionPlan,
    expanded_length: usize,
    // Mapping of coordinates to coordinates
    corner_mappings: HashMap<Coord, Coord>,
}

struct TransformationMappings<'a> {
    tile_expansion_plans: Vec<TileExpansionPlan>,
    mappings: HashMap<SquareMatrix, &'a SquareMatrix>,
}

impl<'a> TransformationMappings<'a> {
    fn from_rules(
        rules: &'a Vec<TransformRule>,
    ) -> Result<TransformationMappings<'a>, &'static str> {
        let tile_expansion_plans = TileExpansionPlan::from_rules(rules)?;
        let mappings = generate_mappings(rules);
        Ok(TransformationMappings {
            tile_expansion_plans,
            mappings,
        })
    }
}

fn generate_mappings(rules: &Vec<TransformRule>) -> HashMap<SquareMatrix, &SquareMatrix> {
    let mut m = HashMap::with_capacity(rules.len() * 12);
    for rule in rules {
        let input_variations = rule.input.variations();
        for variation in input_variations.into_iter() {
            m.insert(variation, &rule.output);
        }
    }
    m
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct TileExpansionPlan {
    from: usize,
    to: usize,
}

impl TileExpansionPlan {
    fn from_rules(rules: &Vec<TransformRule>) -> Result<Vec<TileExpansionPlan>, &'static str> {
        let mut input_len_to_output_lens = HashMap::new();
        for rule in rules {
            input_len_to_output_lens
                .entry(rule.input.length)
                .or_insert(HashSet::new())
                .insert(rule.output.length);
        }
        let mut plans = Vec::with_capacity(input_len_to_output_lens.len());
        for (input_len, output_lens) in input_len_to_output_lens {
            if output_lens.len() != 1 {
                Err("More than 1 output matrix dimension for a given input matrix dimension :(")?
            } else {
                if let Some(output_len) = output_lens.iter().next() {
                    plans.push(TileExpansionPlan {
                        from: input_len,
                        to: *output_len,
                    })
                }
            }
        }
        plans.sort_unstable_by_key(|p| p.from);
        Ok(plans)
    }
}

impl SquareMatrix {
    fn new(data: &Vec<Vec<State>>) -> Result<SquareMatrix, &'static str> {
        let height = data.len();
        let width = data.iter().map(|row| row.len()).next().unwrap_or(0);
        if data.iter().map(|row| row.len()).any(|l| l != width) {
            Err("Not all rows have the same number of columns :(")
        } else if height != width {
            Err("SquareMatrices must be square. Sorry.")
        } else if height == 0 || width == 0 {
            Err("SquareMatrices not be zero size. Sorry.")
        } else {
            let max_i = height - 1;
            let _data = data.iter()
                .fold(Vec::with_capacity(height * width), |acc, row| {
                    row.iter().fold(acc, |mut inner_acc, c| {
                        inner_acc.push(*c);
                        inner_acc
                    })
                });
            Ok(SquareMatrix {
                _data,
                length: height,
                max_i,
            })
        }
    }

    // Can't really use [M; N] array because fixed size arrays don't support
    // into_iter() yet (see https://github.com/rust-lang/rust/issues/25725)
    fn variations(&self) -> Vec<SquareMatrix> {
        let now = self.clone();
        let h_flipped = self.flip_horizontal();
        let v_flipped = self.flip_vertical();
        let rot_90 = self.rotate_right(R90);
        let rot_90_h_flipped = rot_90.flip_horizontal();
        let rot_90_v_flipped = rot_90.flip_vertical();
        let rot_180 = rot_90.rotate_right(R90);
        let rot_180_h_flipped = rot_180.flip_horizontal();
        let rot_180_v_flipped = rot_180.flip_vertical();
        let rot_270 = rot_180.rotate_right(R90);
        let rot_270_h_flipped = rot_270.flip_horizontal();
        let rot_270_v_flipped = rot_270.flip_vertical();
        vec![
            now,
            h_flipped,
            v_flipped,
            rot_90,
            rot_90_h_flipped,
            rot_90_v_flipped,
            rot_180,
            rot_180_h_flipped,
            rot_180_v_flipped,
            rot_270,
            rot_270_h_flipped,
            rot_270_v_flipped,
        ]
    }

    fn to_idx(&self, &Coord { i, j }: &Coord) -> Option<InternalIdx> {
        if i <= self.max_i && j <= self.max_i {
            let idx = self.length * i + j;
            Some(InternalIdx(idx))
        } else {
            None
        }
    }

    fn manipulate_op<F>(&self, f: F) -> SquareMatrix
    where
        F: Fn(Coord) -> Option<(Coord, Coord)>,
    {
        let mut copy = self.clone();
        for i in 0..self.length {
            for j in 0..self.length {
                if let Some((coord1, coord2)) = f(Coord { i, j }) {
                    if let Some(InternalIdx(idx1)) = self.to_idx(&coord1) {
                        if let Some(InternalIdx(idx2)) = self.to_idx(&coord2) {
                            copy._data[idx1] = self._data[idx2];
                            copy._data[idx2] = self._data[idx1];
                        }
                    }
                }
            }
        }
        copy
    }

    fn flip_horizontal(&self) -> SquareMatrix {
        self.manipulate_op(|Coord { i, j }| {
            Some((
                Coord { i: i, j: j },
                Coord {
                    i: self.max_i - i,
                    j: j,
                },
            ))
        })
    }

    fn flip_vertical(&self) -> SquareMatrix {
        self.manipulate_op(|Coord { i, j }| {
            Some((
                Coord { i: i, j: j },
                Coord {
                    i: i,
                    j: self.max_i - j,
                },
            ))
        })
    }

    fn transpose(&self) -> SquareMatrix {
        self.manipulate_op(|Coord { i, j }| {
            if i != j {
                Some((Coord { i: i, j: j }, Coord { i: j, j: i }))
            } else {
                None
            }
        })
    }

    fn rotate_right(&self, rotation: Rotation) -> SquareMatrix {
        match rotation {
            R90 => self.transpose().flip_horizontal(),
            R180 => self.flip_horizontal().flip_vertical(),
            R270 => self.transpose().flip_vertical(),
            R360 => self.clone(),
        }
    }

    fn get_square(&self, upper_left: Coord, length: usize) -> Option<SquareMatrix> {
        let bottom_right = Coord {
            i: upper_left.i + length - 1,
            j: upper_left.j + length - 1,
        };
        if upper_left >= MIN_COORD && bottom_right.i <= self.max_i && bottom_right.j <= self.max_i {
            let mut _data = Vec::with_capacity(length * length);
            for i in upper_left.i..upper_left.i + length {
                let InternalIdx(left_idx) = self.to_idx(&Coord {
                    i: i,
                    j: upper_left.j,
                })?;
                let InternalIdx(right_idx) = self.to_idx(&Coord {
                    i: i,
                    j: bottom_right.j,
                })?;
                _data.extend_from_slice(&self._data[left_idx..right_idx + 1]);
            }
            Some(SquareMatrix {
                _data: _data,
                length: length,
                max_i: length - 1,
            })
        } else {
            None
        }
    }

    fn set_square(&mut self, upper_left: Coord, other: &SquareMatrix) -> Result<(), &'static str> {
        let bottom_right = Coord {
            i: upper_left.i + other.length - 1,
            j: upper_left.j + other.length - 1,
        };
        if upper_left >= MIN_COORD && bottom_right.i <= self.max_i && bottom_right.j <= self.max_i {
            for (other_i, i) in (upper_left.i..upper_left.i + other.length).enumerate() {
                let InternalIdx(left_idx) = opt_to_result(self.to_idx(&Coord {
                    i: i,
                    j: upper_left.j,
                }))?;
                let InternalIdx(right_idx) = opt_to_result(self.to_idx(&Coord {
                    i: i,
                    j: bottom_right.j,
                }))?;
                let InternalIdx(other_left_idx) =
                    opt_to_result(other.to_idx(&Coord { i: other_i, j: 0 }))?;
                let InternalIdx(other_right_idx) = opt_to_result(other.to_idx(&Coord {
                    i: other_i,
                    j: other.max_i,
                }))?;
                self._data[left_idx..right_idx + 1]
                    .copy_from_slice(&other._data[other_left_idx..other_right_idx + 1]);
            }
            Ok(())
        } else {
            Err("The coordinates you gave and the other matrix don't inside this one.")
        }
    }

    fn expand(
        &mut self,
        transformation_mappings: &TransformationMappings,
        times: usize,
    ) -> Result<(), &'static str> {
        for _ in 0..times {
            let original = self.clone();
            let expansion_plan =
                self.expansion_data(&transformation_mappings.tile_expansion_plans)?;
            self.expand_to(expansion_plan.expanded_length, Off)?;
            for (input_upper_left, output_upper_left) in expansion_plan.corner_mappings.iter() {
                let input_square = opt_to_result(
                    original.get_square(*input_upper_left, expansion_plan.plan.from),
                )?;
                let output_square =
                    opt_to_result(transformation_mappings.mappings.get(&input_square))?;
                self.set_square(*output_upper_left, *output_square)?;
            }
        }
        Ok(())
    }

    fn expansion_data(
        &self,
        expansion_plans: &Vec<TileExpansionPlan>,
    ) -> Result<ExpansionData, &'static str> {
        match expansion_plans.iter().find(|d| self.length % d.from == 0) {
            Some(expansion_plan) => {
                let tile_expansion_diff = expansion_plan.to - expansion_plan.from;
                let length_tile_count = self.length / expansion_plan.from;
                let expanded_length = length_tile_count * expansion_plan.to;
                let mut corner_mappings =
                    HashMap::with_capacity(length_tile_count * length_tile_count);
                let mut current_i = 0;
                let mut current_i_idx = 0;
                while current_i < self.length {
                    let mut current_j = 0;
                    let mut current_j_idx = 0;
                    while current_j < self.length {
                        corner_mappings.insert(
                            Coord {
                                i: current_i,
                                j: current_j,
                            },
                            Coord {
                                i: current_i + current_i_idx * tile_expansion_diff,
                                j: current_j + current_j_idx * tile_expansion_diff,
                            },
                        );
                        current_j += expansion_plan.from;
                        current_j_idx += 1;
                    }
                    current_i += expansion_plan.from;
                    current_i_idx += 1;
                }

                Ok(ExpansionData {
                    plan: *expansion_plan,
                    expanded_length,
                    corner_mappings,
                })
            }
            None => Err("Cannot find usable expansion points"),
        }
    }

    fn expand_to(&mut self, length: usize, fill_with: State) -> Result<(), &'static str> {
        let new_data_length = length * length;
        if length == 0 {
            // Shouldn't really happen but meh
            Err("Cannot expand to zero length.")
        } else if new_data_length <= self._data.len() {
            Err("Cannot expand to a length equal or less than the current size of the SquareMatrix")
        } else {
            self._data.resize(new_data_length, fill_with);
            self.length = length;
            self.max_i = length - 1;
            Ok(())
        }
    }
}

fn opt_to_result<T>(opt: Option<T>) -> Result<T, &'static str> {
    match opt {
        Some(t) => Ok(t),
        None => Err("Was NONE!"),
    }
}

#[derive(Debug)]
struct TransformRule {
    input: SquareMatrix,
    output: SquareMatrix,
}

macro_rules! states_parser {
    () => {
        many1(
            try(char('.').map(|_| Off))
            .or(char('#').map(|_| On))
            )
    }
}

macro_rules! vec_states_parser {
    () => {
        sep_by(
            states_parser!(),
            char('/')
        )
    }
}

macro_rules! transform_rule_parser {
    () => {
        vec_states_parser!()
            .skip(tabs_or_spaces!().and(string("=>")).and(tabs_or_spaces!()))
            .and(vec_states_parser!())
            .and_then(|(input_v, output_v)| {
                SquareMatrix::new(&input_v).and_then(|input| {
                    SquareMatrix::new(&output_v).map(|output| {
                        TransformRule { input, output }
                    })
                }).map_err(|e| Error::Message(easy::Info::Borrowed(e)))
            })
    }
}

impl SquareMatrix {
    fn parse(s: &str) -> Result<SquareMatrix, Errors<PointerOffset, char, &str>> {
        let mut parser = vec_states_parser!().and_then(|v| {
            SquareMatrix::new(&v).map_err(|e| Error::Message(easy::Info::Borrowed(e)))
        });
        let (matrix, _) = parser.easy_parse(s)?;
        Ok(matrix)
    }
}

impl TransformRule {
    fn parse_many(s: &str) -> Result<Vec<TransformRule>, Errors<PointerOffset, char, &str>> {
        let mut parser = (tabs_or_spaces!()).with(sep_by(transform_rule_parser!(), spaces()));
        let (rules, _) = parser.easy_parse(s)?;
        Ok(rules)
    }
}

#[cfg(test)]
mod tests {
    use day_21::*;

    #[test]
    fn flip_horizontal_test() {
        let m = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let m2 = m.flip_horizontal();
        let expected = SquareMatrix::new(&vec![
            vec![Off, Off, On],
            vec![Off, On, Off],
            vec![On, Off, Off],
        ]).unwrap();
        assert_eq!(m2, expected);
    }

    #[test]
    fn flip_vertical_test() {
        let m = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let m2 = m.flip_vertical();
        let expected = SquareMatrix::new(&vec![
            vec![Off, Off, On],
            vec![Off, On, Off],
            vec![On, Off, Off],
        ]).unwrap();
        assert_eq!(m2, expected);
    }

    #[test]
    fn flip_transpose_test() {
        let m = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let m2 = m.transpose();
        assert_eq!(m2, m);
    }

    #[test]
    fn rotate_right_test() {
        let m = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let m2 = m.rotate_right(R90);
        let expected1 = SquareMatrix::new(&vec![
            vec![Off, Off, On],
            vec![Off, On, Off],
            vec![On, Off, Off],
        ]).unwrap();
        assert_eq!(m2, expected1);
        let m3 = m.rotate_right(R180);
        let expected2 = expected1.rotate_right(R90);
        assert_eq!(m3, expected2);
        let m4 = m.rotate_right(R270);
        let expected3 = expected2.rotate_right(R90);
        assert_eq!(m4, expected3);
        let m5 = m.rotate_right(R360);
        assert_eq!(m5, m);
    }

    #[test]
    fn get_square_test() {
        let m = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let s1 = m.get_square(Coord { i: 0, j: 0 }, 3).unwrap();
        let s2 = m.get_square(Coord { i: 0, j: 0 }, 2).unwrap();
        assert_eq!(s1, m);
        assert_eq!(
            s2,
            SquareMatrix::new(&vec![vec![On, Off], vec![Off, On]]).unwrap()
        );
    }

    #[test]
    fn set_square_test() {
        let mut m1 = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let m2 = SquareMatrix::new(&vec![vec![On, On], vec![On, On]]).unwrap();
        m1.set_square(Coord { i: 0, j: 0 }, &m2).unwrap();
        let expected = SquareMatrix::new(&vec![
            vec![On, On, Off],
            vec![On, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        assert_eq!(m1, expected);
    }

    #[test]
    fn transform_rules_parse_test() {
        let parsed = TransformRule::parse_many(DAY_21_RULES).unwrap();
        assert_eq!(parsed.len(), 108);
    }

    #[test]
    fn expand_to_test() {
        let mut m1 = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        m1.expand_to(4, Off).unwrap();
        let expected = SquareMatrix::new(&vec![
            vec![On, Off, Off, Off],
            vec![On, Off, Off, Off],
            vec![On, Off, Off, Off],
            vec![Off, Off, Off, Off],
        ]).unwrap();
        assert_eq!(m1, expected);
    }

    #[test]
    fn expansion_plan_test() {
        let m1 = SquareMatrix::new(&vec![
            vec![On, Off, Off],
            vec![Off, On, Off],
            vec![Off, Off, On],
        ]).unwrap();
        let expansion_plans = vec![
            TileExpansionPlan { from: 2, to: 3 },
            TileExpansionPlan { from: 3, to: 4 },
        ];
        let expansion_data1 = m1.expansion_data(&expansion_plans).unwrap();
        let expected1 = ExpansionData {
            plan: TileExpansionPlan { from: 3, to: 4 },
            expanded_length: 4,
            corner_mappings: hashmap![
                Coord { i: 0, j: 0 } => Coord { i: 0, j: 0 }
            ],
        };
        assert_eq!(expansion_data1, expected1);
        let m2 = SquareMatrix::new(&vec![
            vec![On, Off, Off, On],
            vec![Off, On, Off, Off],
            vec![Off, Off, On, On],
            vec![Off, Off, On, On],
        ]).unwrap();
        let expansion_data2 = m2.expansion_data(&expansion_plans).unwrap();
        let expected2 = ExpansionData {
            plan: TileExpansionPlan { from: 2, to: 3 },
            expanded_length: 6,
            corner_mappings: hashmap![
                Coord { i: 0, j: 0 } => Coord { i: 0, j: 0 },
                Coord { i: 0, j: 2 } => Coord { i: 0, j: 3 },
                Coord { i: 2, j: 0 } => Coord { i: 3, j: 0 },
                Coord { i: 2, j: 2 } => Coord { i: 3, j: 3 },
            ],
        };
        assert_eq!(expansion_data2, expected2);
    }

    #[test]
    fn solution_1_test() {
        let mut original_matrix = SquareMatrix::parse(DAY_21_MATRIX).unwrap();
        let parsed_rules = TransformRule::parse_many(DAY_21_RULES).unwrap();
        let mappings = TransformationMappings::from_rules(&parsed_rules).unwrap();
        assert_eq!(solution(&mut original_matrix, &mappings, 5).unwrap(), 186);
    }

    #[test]
    fn solution_2_test() {
        let mut original_matrix = SquareMatrix::parse(DAY_21_MATRIX).unwrap();
        let parsed_rules = TransformRule::parse_many(DAY_21_RULES).unwrap();
        let mappings = TransformationMappings::from_rules(&parsed_rules).unwrap();
        assert_eq!(
            solution(&mut original_matrix, &mappings, 18).unwrap(),
            3018423
        );
    }
}
