//! Hex Coord system
//!   + -2,2 +--+ 0,1  +--+  2,0 +--+ 4,-2 +
//!    \    /    \    /    \    /    \    /
//!     +--+ -1,1 +--+ 1,0  +--+ 3,-2 +--+
//!    /    \    /    \    /    \    /    \
//!  -+ -2,0 +--+ 0,0  +--+ 2,-1 +--+ 4,-2 +
//!    \    /    \    /    \    /    \    /
//!     +--+ -1,0 +--+ 1,-1 +--+ 3,-2 +--+
//!    /    \    /    \    /    \    /    \
//!   + -2,0 +--+ 0,-1 +--+ 2,-2 +--+ 4,-3 +
//!   \     /    \    /    \    /    \    /
//!     +--+-1,-1 +--+ 1,-2 +--+ 3,-3 +--+
//! In other words
//!
//! Stepping:
//!  N  -> (current_a    , current_b + 1)
//!  NE -> (current_a + 1, current_b    )
//!  SE -> (current_a + 1, current_b - 1)
//!  NW -> (current_a - 1, current_b + 1)
//!  SW -> (current_a - 1, current_b    )
//!  S  -> (current_a    , current_b - 1)

use std::str::FromStr;

const DAY_11_INPUT: &'static str = include_str!("../data/day_11_input");
const HEX_AXIAL_ORIGIN: HexAxialCoord = HexAxialCoord { q: 0, r: 0 };

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 11: Hex Ed ***");
    println!("Input: {}", DAY_11_INPUT);
    println!("Solution1: {:?}\n", hex_steps_from_centre(DAY_11_INPUT));
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct GeoTracker {
    current: HexAxialCoord,
    current_distance: usize,
    farthest_distance: usize,
}

fn hex_steps_from_centre(s: &str) -> GeoTracker {
    let as_steps = s.split(",").filter_map(|s| s.trim().parse().ok());
    as_steps.fold(
        GeoTracker {
            current: HEX_AXIAL_ORIGIN,
            current_distance: 0,
            farthest_distance: 0,
        },
        |acc, next| {
            let current = acc.current.step(next);
            let current_distance = HEX_AXIAL_ORIGIN.steps_from(&current);
            let farthest_distance = if current_distance > acc.farthest_distance {
                current_distance
            } else {
                acc.farthest_distance
            };
            GeoTracker {
                current,
                current_distance,
                farthest_distance,
            }
        },
    )
}

enum Step {
    N,
    NE,
    SE,
    NW,
    SW,
    S,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Step::*;
        match s.to_lowercase().as_ref() {
            "n" => Ok(N),
            "ne" => Ok(NE),
            "se" => Ok(SE),
            "nw" => Ok(NW),
            "sw" => Ok(SW),
            "s" => Ok(S),
            other => Err(format!("Invalid hex step: {}", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct HexAxialCoord {
    q: isize,
    r: isize,
}

impl HexAxialCoord {
    fn as_cube(&self) -> HexCubeCoord {
        HexCubeCoord {
            x: self.q,
            y: -self.q - self.r,
            z: self.r,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct HexCubeCoord {
    x: isize,
    y: isize,
    z: isize,
}

trait StepsDistance {
    fn steps_from(&self, other: &Self) -> usize;
}

impl StepsDistance for HexCubeCoord {
    #[inline]
    fn steps_from(&self, other: &HexCubeCoord) -> usize {
        let max = (self.x - other.x)
            .abs()
            .max((self.y - other.y).abs().max((self.z - other.z).abs()));
        max as usize
    }
}

impl StepsDistance for HexAxialCoord {
    #[inline]
    fn steps_from(&self, other: &HexAxialCoord) -> usize {
        self.as_cube().steps_from(&other.as_cube())
    }
}

trait Walk {
    fn step(&self, step: Step) -> Self;
}

impl Walk for HexAxialCoord {
    fn step(&self, step: Step) -> HexAxialCoord {
        let current_a = self.q;
        let current_b = self.r;
        match step {
            Step::N => HexAxialCoord {
                q: current_a,
                r: current_b + 1,
            },
            Step::NE => HexAxialCoord {
                q: current_a + 1,
                r: current_b,
            },
            Step::SE => HexAxialCoord {
                q: current_a + 1,
                r: current_b - 1,
            },
            Step::NW => HexAxialCoord {
                q: current_a - 1,
                r: current_b + 1,
            },
            Step::SW => HexAxialCoord {
                q: current_a - 1,
                r: current_b,
            },
            Step::S => HexAxialCoord {
                q: current_a,
                r: current_b - 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use day_11::*;
    use day_11::Step::*;

    #[test]
    fn step_test() {
        let coord1 = HexAxialCoord { q: 0, r: 0 };
        let coord2 = coord1.step(N);
        let coord3 = coord2.step(NE);
        let coord4 = coord3.step(S);
        let coord5 = coord4.step(SW);
        let coord6 = coord5
            .step(NE)
            .step(NE)
            .step(N)
            .step(N)
            .step(SW)
            .step(S)
            .step(SE)
            .step(N)
            .step(SW)
            .step(NW)
            .step(S)
            .step(S);
        let coord7 = coord1.step(SE).step(SW).step(SE).step(SW).step(SW);
        assert_eq!(coord2, HexAxialCoord { q: 0, r: 1 });
        assert_eq!(coord3, HexAxialCoord { q: 1, r: 1 });
        assert_eq!(coord4, HexAxialCoord { q: 1, r: 0 });
        assert_eq!(coord5, HexAxialCoord { q: 0, r: 0 });
        assert_eq!(coord6, HexAxialCoord { q: 0, r: 0 });
        assert_eq!(coord7, HexAxialCoord { q: -1, r: -2 });
    }

    #[test]
    fn distance_test() {
        assert_eq!(
            HexAxialCoord { q: 1, r: 0 }.steps_from(&HexAxialCoord { q: 1, r: -1 }),
            1
        );
        assert_eq!(
            HexAxialCoord { q: 1, r: 0 }.steps_from(&HexAxialCoord { q: 0, r: 0 }),
            1
        );
        assert_eq!(
            HexAxialCoord { q: 1, r: 0 }.steps_from(&HexAxialCoord { q: 0, r: -1 }),
            2
        );
        assert_eq!(
            HexAxialCoord { q: 1, r: 0 }.steps_from(&HexAxialCoord { q: 0, r: -2 }),
            3
        );
    }

    #[test]
    fn first_half_test() {
        assert_eq!(hex_steps_from_centre(DAY_11_INPUT).current_distance, 759);
    }

    #[test]
    fn second_half_test() {
        assert_eq!(hex_steps_from_centre(DAY_11_INPUT).farthest_distance, 1501);
    }
}
