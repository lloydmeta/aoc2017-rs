use std::usize;
use std::collections::HashMap;
use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;
use rayon::prelude::*;
use num_integer::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Depth(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Picoseconds(usize);

#[derive(Debug, PartialEq, Eq)]
struct Layer {
    depth: Depth,
    range: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ScannerDirection {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct LayerState<'a> {
    layer: &'a Layer,
    current_direction: ScannerDirection, // useful for debugging
    current_location: usize,
}

#[derive(Debug, Clone)]
struct LayerStates<'a>(HashMap<Depth, LayerState<'a>>);

#[derive(Debug, Clone)]
struct GameState<'a> {
    current_time: Picoseconds,
    current_depth: Depth,
    layers: &'a Layers,
    layer_states: LayerStates<'a>,
    caught_in: Vec<Depth>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct GameResult {
    times_caught: usize,
    total_severity: usize,
}

pub fn calculate_trip_result(s: &str) -> Result<GameResult, Errors<PointerOffset, char, &str>> {
    let (layers, _) = Layers::parse(s)?;
    let mut game = GameState::from(&layers);
    Ok(game.play())
}

pub fn find_uncaught_delay(
    s: &str,
) -> Result<Option<Picoseconds>, Errors<PointerOffset, char, &str>> {
    let (layers, _) = Layers::parse(s)?;
    let clean_state = GameState::from(&layers);
    let maybe_viable_delay = (0..usize::max_value())
        .into_par_iter()
        .map(|d| {
            let delay = Picoseconds(d);
            let mut game = clean_state.clone();
            game.advance_to(delay);
            (delay, game.play())
        })
        .find_first(|&(_, ref result)| result.times_caught == 0)
        .map(|(d, _)| d);
    Ok(maybe_viable_delay)
}

impl<'a> GameState<'a> {
    fn play(&mut self) -> GameResult {
        let highest_depth = self.layers
            .0
            .iter()
            .fold(Depth(0), |acc, next| if next.depth > acc {
                next.depth
            } else {
                acc
            });
        while self.current_depth <= highest_depth {
            self.advance();
        }
        let init_result = GameResult {
            times_caught: self.caught_in.len(),
            total_severity: 0,
        };
        self.caught_in.iter().fold(init_result, |mut acc, depth| {
            if let Some(l_state) = self.layer_states.0.get(depth) {
                acc.total_severity = acc.total_severity + depth.0 * l_state.layer.range;
            }
            acc
        })
    }

    fn from(layers: &'a Layers) -> GameState {
        let layer_states = LayerStates::from(layers);
        GameState {
            current_time: Picoseconds(0),
            current_depth: Depth(0),
            layers: layers,
            layer_states: layer_states,
            caught_in: Vec::new(),
        }
    }

    fn advance(&mut self) -> () {
        match self.layer_states.scanner_location(self.current_depth) {
            Some(0) => {
                self.caught_in.push(self.current_depth);
            }
            _ => (),
        }
        self.current_time.0 += 1;
        self.layer_states.advance_to(self.current_time);
        self.current_depth.0 += 1;
    }

    // Time travels the layer_states to a given point in time,
    // and set the self time to a given point.
    fn advance_to(&mut self, time: Picoseconds) -> () {
        self.layer_states.advance_to(time);
        self.current_time = time;
    }
}

impl<'a> LayerState<'a> {
    fn from(layer: &'a Layer) -> LayerState {
        use day_13::ScannerDirection::*;
        LayerState {
            layer: layer,
            current_direction: Down,
            current_location: 0,
        }
    }

    fn advance_to(&mut self, Picoseconds(time): Picoseconds) -> () {
        use day_13::ScannerDirection::*;
        if self.layer.range > 1 {
            let highest_location = self.layer.range - 1;
            //  range | t 0 1 2 3 4 5 6 7 8 9 10
            //    1       0 0 0 0 0 0 0 0 0 0 0
            //    2       0 1 0 1 0 1 0 1 0 1 0
            //    3       0 1 2 1 0 1 2 1 0 1 2
            //    4       0 1 2 3 2 1 0 1 2 3 2
            let (div, modulus) = time.div_rem(&highest_location);
            if div % 2 == 0 {
                self.current_direction = Down;
                if modulus == 0 {
                    self.current_location = 0;
                } else {
                    self.current_location = modulus;
                }
            } else {
                // Odd div
                self.current_direction = Up;
                if modulus == 0 {
                    // no remainder
                    self.current_location = highest_location;
                } else {
                    // with remainder -> going back down fom top
                    self.current_location = highest_location - modulus;
                }
            }
        }
    }
}


impl<'a> LayerStates<'a> {
    fn scanner_location(&self, depth: Depth) -> Option<usize> {
        self.0.get(&depth).map(|ls| ls.current_location)
    }

    fn from(layers: &'a Layers) -> LayerStates<'a> {
        let layer_states = layers
            .0
            .iter()
            .map(|l| (l.depth, LayerState::from(l)))
            .collect();
        LayerStates(layer_states)
    }

    fn advance_to(&mut self, time: Picoseconds) -> () {
        for (_, layer_state) in &mut self.0 {
            layer_state.advance_to(time);
        }
    }
}

macro_rules! layer_parser {
    () => {
        {
            pos_number_parser!(usize).map(|d| Depth(d))
                .skip(tabs_or_spaces!())
                .skip(char(':'))
                .skip(tabs_or_spaces!())
                .and(pos_number_parser!(usize))
                .map(|(depth, range)| Layer { depth, range })
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
struct Layers(Vec<Layer>);

impl Layers {
    fn parse(s: &str) -> Result<(Layers, &str), Errors<PointerOffset, char, &str>> {
        let mut parser = skip_many(newline())
            .with(sep_by(layer_parser!(), spaces()))
            .map(|layers| Layers(layers));
        parser.easy_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use day_13::*;

    const TEST_INPUT: &'static str = r#"
0: 3
1: 2
4: 4
6: 4"#;

    #[test]
    fn layers_parse_test() {
        let (layers, _) = Layers::parse(DAY_13_INPUT).unwrap();
        assert!(layers.0.len() > 0);
    }

    #[test]
    fn test_input_parse_test() {
        let (layers, _) = Layers::parse(TEST_INPUT).unwrap();
        assert_eq!(layers.0.len(), 4);
    }

    #[test]
    fn layer_state_advance_test() {
        let layer = Layer {
            depth: Depth(0),
            range: 3,
        };
        let mut layer_state = LayerState::from(&layer);
        assert_eq!(layer_state.current_location, 0);
        layer_state.advance_to(Picoseconds(1));
        assert_eq!(layer_state.current_location, 1);
        layer_state.advance_to(Picoseconds(2));
        assert_eq!(layer_state.current_location, 2);
        layer_state.advance_to(Picoseconds(3));
        assert_eq!(layer_state.current_location, 1);
        layer_state.advance_to(Picoseconds(4));
        assert_eq!(layer_state.current_location, 0);
        layer_state.advance_to(Picoseconds(5));
        assert_eq!(layer_state.current_location, 1);
        for t in 6..1000 {
            // sanity check
            layer_state.advance_to(Picoseconds(t));
        }
    }

    #[test]
    fn layer_states_advance_to_test() {
        let layers = Layers(vec![
            Layer {
                depth: Depth(0),
                range: 3,
            },
            Layer {
                depth: Depth(1),
                range: 2,
            },
            Layer {
                depth: Depth(3),
                range: 5,
            },
        ]);
        let mut layer_states = LayerStates::from(&layers);
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(0));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(0));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(0));
        layer_states.advance_to(Picoseconds(1));
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(1));
        layer_states.advance_to(Picoseconds(2));
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(2));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(0));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(2));
        layer_states.advance_to(Picoseconds(3));
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(3));
        layer_states.advance_to(Picoseconds(4));
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(0));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(0));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(4));
        layer_states.advance_to(Picoseconds(5));
        assert_eq!(layer_states.scanner_location(Depth(0)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(1)), Some(1));
        assert_eq!(layer_states.scanner_location(Depth(2)), None);
        assert_eq!(layer_states.scanner_location(Depth(3)), Some(3));
    }

    #[test]
    fn part_1_test() {
        let r = calculate_trip_result(TEST_INPUT).unwrap();
        assert_eq!(
            r,
            GameResult {
                times_caught: 2,
                total_severity: 24,
            }
        );
    }

    #[test]
    fn part1_real_test() {
        let r = calculate_trip_result(DAY_13_INPUT).unwrap();
        assert_eq!(r.total_severity, 1900);
    }

    #[test]
    fn find_uncaught_delay_test() {
        let r = find_uncaught_delay(TEST_INPUT).unwrap().unwrap();
        assert_eq!(r, Picoseconds(10));
    }
}

pub const DAY_13_INPUT: &str = r#"
0: 3
1: 2
2: 4
4: 4
6: 5
8: 6
10: 6
12: 6
14: 6
16: 8
18: 8
20: 8
22: 8
24: 10
26: 8
28: 8
30: 12
32: 14
34: 12
36: 10
38: 12
40: 12
42: 9
44: 12
46: 12
48: 12
50: 12
52: 14
54: 14
56: 14
58: 12
60: 14
62: 14
64: 12
66: 14
70: 14
72: 14
74: 14
76: 14
80: 18
88: 20
90: 14
98: 17"#;
