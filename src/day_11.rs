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

const HEX_AXIAL_ORIGIN: HexAxialCoord = HexAxialCoord { q: 0, r: 0 };

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GeoTracker {
    current: HexAxialCoord,
    current_distance: usize,
    farthest_distance: usize,
}

pub fn hex_steps_from_centre(s: &str) -> GeoTracker {
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
pub struct HexAxialCoord {
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

pub const DAY_11_INPUT: &'static str = r#"sw,nw,nw,se,ne,ne,n,ne,n,ne,ne,ne,se,se,se,se,ne,se,se,nw,se,sw,s,se,sw,s,se,s,s,se,s,s,s,n,s,s,s,ne,s,s,s,s,s,sw,sw,ne,n,sw,ne,sw,sw,sw,sw,sw,s,sw,sw,s,sw,sw,sw,sw,n,sw,nw,sw,sw,nw,s,sw,sw,s,nw,nw,nw,sw,nw,nw,nw,nw,nw,sw,sw,ne,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,sw,nw,nw,se,nw,nw,nw,n,n,nw,nw,nw,nw,nw,nw,s,se,nw,nw,s,nw,se,n,n,n,nw,nw,nw,nw,n,se,nw,s,n,nw,n,n,ne,n,n,nw,n,n,n,n,se,n,ne,n,n,n,n,se,n,ne,n,n,n,n,n,n,sw,n,n,n,ne,n,ne,ne,nw,ne,ne,ne,n,ne,ne,n,se,ne,ne,ne,sw,n,sw,n,n,nw,n,ne,ne,se,ne,ne,nw,ne,n,s,ne,n,ne,n,ne,ne,n,ne,se,n,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,nw,ne,ne,nw,ne,ne,sw,nw,ne,ne,ne,sw,ne,ne,ne,ne,ne,se,s,ne,ne,se,se,ne,ne,se,se,ne,nw,ne,se,ne,ne,ne,ne,se,s,ne,ne,sw,ne,s,ne,ne,se,se,n,ne,se,ne,se,se,se,n,ne,se,ne,ne,se,ne,se,se,n,ne,sw,ne,se,se,n,se,ne,se,ne,se,nw,se,se,se,s,se,se,se,se,se,se,se,se,nw,ne,ne,s,se,se,se,se,se,se,se,se,n,ne,se,n,s,se,se,se,se,ne,se,nw,se,se,s,se,se,s,s,s,se,se,s,se,s,n,se,se,se,se,nw,se,se,se,sw,se,s,se,se,se,nw,s,se,s,se,se,se,ne,se,se,se,se,se,sw,n,s,s,sw,s,s,se,se,se,se,s,s,ne,s,nw,ne,se,se,se,s,s,s,sw,s,sw,ne,se,ne,s,s,se,se,se,s,se,se,n,s,n,se,se,se,se,s,se,se,se,n,n,se,s,s,ne,se,s,s,n,se,sw,s,s,s,ne,s,s,s,ne,s,s,s,s,nw,sw,ne,ne,sw,s,s,s,s,nw,s,s,n,s,s,s,s,s,nw,s,s,s,s,sw,s,s,s,s,sw,nw,s,s,s,ne,sw,se,s,se,sw,s,s,s,s,s,ne,s,sw,se,s,s,n,se,sw,s,sw,s,s,s,se,s,sw,s,s,s,s,s,s,s,s,sw,s,s,s,sw,s,s,s,s,s,sw,s,sw,s,s,se,s,sw,n,sw,s,sw,s,sw,sw,sw,s,sw,sw,sw,n,s,se,nw,ne,sw,se,sw,nw,sw,s,s,sw,s,n,sw,s,s,ne,sw,sw,s,s,s,s,sw,se,ne,sw,sw,sw,s,s,s,s,sw,s,n,sw,s,sw,s,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,ne,sw,se,s,sw,s,sw,sw,nw,sw,n,sw,sw,sw,sw,n,nw,s,ne,s,sw,sw,sw,sw,n,sw,ne,sw,se,s,sw,sw,sw,sw,s,sw,sw,sw,sw,n,ne,s,s,sw,sw,s,ne,sw,sw,s,sw,sw,se,sw,sw,sw,ne,sw,sw,sw,se,n,s,sw,s,sw,sw,sw,se,sw,sw,n,sw,sw,sw,sw,se,sw,sw,sw,n,ne,sw,sw,sw,s,sw,sw,sw,sw,sw,nw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,n,nw,sw,sw,sw,ne,sw,se,sw,ne,nw,sw,sw,s,sw,sw,sw,nw,ne,sw,sw,sw,sw,se,sw,sw,sw,sw,nw,sw,ne,sw,sw,sw,sw,se,ne,sw,sw,sw,sw,sw,ne,nw,sw,nw,sw,sw,nw,nw,sw,sw,nw,sw,sw,nw,s,sw,se,sw,nw,sw,nw,sw,ne,sw,sw,s,sw,sw,nw,n,nw,n,sw,nw,sw,sw,n,sw,nw,sw,sw,nw,sw,nw,ne,nw,nw,ne,sw,n,nw,nw,sw,nw,sw,sw,sw,nw,s,nw,sw,nw,nw,sw,nw,ne,nw,sw,nw,sw,nw,nw,sw,sw,sw,sw,sw,sw,s,nw,sw,sw,nw,sw,nw,nw,sw,nw,ne,s,s,s,n,sw,sw,sw,nw,sw,sw,sw,sw,nw,nw,nw,sw,ne,sw,nw,sw,ne,sw,nw,se,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,n,sw,nw,se,nw,se,n,sw,se,n,sw,nw,nw,s,nw,nw,nw,nw,ne,nw,nw,nw,sw,nw,nw,nw,nw,se,nw,sw,nw,sw,ne,se,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,se,nw,sw,nw,nw,se,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,nw,n,sw,nw,nw,ne,nw,nw,nw,nw,nw,nw,sw,ne,nw,nw,nw,nw,nw,ne,nw,nw,n,nw,ne,sw,nw,nw,nw,nw,nw,sw,s,nw,nw,nw,nw,se,nw,nw,nw,se,s,nw,se,nw,se,sw,n,nw,s,nw,n,nw,nw,nw,n,nw,ne,nw,nw,nw,s,n,nw,nw,se,nw,nw,n,nw,nw,nw,nw,sw,nw,nw,nw,ne,ne,n,n,nw,nw,nw,n,nw,nw,n,n,nw,sw,nw,nw,nw,nw,nw,ne,nw,nw,n,nw,nw,n,nw,nw,nw,n,n,ne,nw,nw,n,nw,nw,n,nw,nw,n,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,nw,nw,se,n,nw,nw,nw,n,n,nw,nw,n,n,n,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,sw,n,nw,nw,nw,se,n,nw,nw,se,n,n,nw,n,n,n,nw,nw,nw,sw,n,nw,n,nw,n,nw,sw,nw,n,nw,nw,s,s,n,n,nw,nw,se,n,n,sw,n,n,sw,nw,nw,nw,n,s,ne,nw,nw,nw,sw,nw,ne,n,nw,n,n,n,nw,n,nw,nw,n,n,nw,n,nw,n,nw,nw,nw,ne,nw,nw,s,s,sw,se,n,n,nw,nw,ne,nw,nw,nw,n,nw,n,n,n,se,nw,n,n,n,n,ne,n,nw,nw,nw,n,n,sw,nw,n,sw,nw,n,n,n,nw,n,nw,nw,nw,nw,n,n,n,n,nw,nw,n,n,n,n,n,n,n,nw,nw,n,nw,n,n,nw,s,n,n,n,n,nw,nw,nw,n,se,nw,nw,n,n,n,n,n,n,nw,nw,n,nw,nw,n,n,n,nw,nw,n,n,nw,n,n,nw,n,n,sw,nw,se,n,ne,n,nw,ne,nw,n,n,n,n,n,n,n,ne,n,n,n,se,n,se,n,n,nw,n,n,sw,n,n,n,s,n,n,n,nw,nw,n,n,n,n,nw,nw,n,n,nw,n,n,n,n,n,n,n,n,nw,n,n,n,n,n,n,n,n,n,n,n,ne,n,n,n,n,n,n,n,n,n,ne,nw,nw,nw,n,n,se,n,n,n,n,n,n,n,n,n,se,s,n,n,n,n,nw,sw,n,n,n,n,n,n,n,n,n,nw,nw,n,n,ne,ne,n,n,n,n,n,n,n,s,s,n,n,n,s,ne,n,n,n,n,n,n,n,n,n,n,n,nw,n,n,n,n,ne,nw,n,n,ne,n,n,n,n,n,n,ne,n,n,n,n,ne,n,n,n,n,ne,n,ne,n,ne,n,n,ne,sw,n,nw,s,n,n,s,se,sw,n,ne,n,n,n,n,n,sw,n,sw,n,n,n,n,n,n,ne,n,sw,n,n,ne,n,ne,se,n,n,n,n,ne,n,n,n,ne,n,ne,ne,n,n,n,ne,sw,se,n,n,n,n,n,n,ne,ne,n,n,n,n,ne,s,ne,ne,n,n,n,n,n,n,n,ne,se,n,ne,nw,n,nw,se,n,n,ne,n,ne,n,n,sw,ne,n,n,ne,se,n,n,ne,n,n,n,n,n,n,n,n,n,sw,n,n,s,nw,n,se,ne,n,ne,ne,se,n,n,n,sw,sw,n,n,n,s,n,s,sw,n,n,n,n,n,n,n,se,ne,n,ne,n,ne,ne,nw,n,n,n,s,ne,n,se,se,ne,s,n,n,ne,nw,s,ne,se,n,n,sw,ne,s,n,ne,se,n,ne,n,ne,n,s,n,n,ne,ne,ne,se,se,ne,n,ne,n,n,n,s,n,sw,ne,ne,ne,nw,ne,ne,n,ne,n,ne,n,n,ne,sw,ne,ne,ne,sw,n,n,ne,ne,n,se,ne,n,ne,nw,ne,n,ne,s,n,n,se,n,ne,ne,n,sw,nw,ne,ne,n,ne,n,n,se,n,se,sw,sw,n,ne,ne,ne,nw,ne,ne,n,n,n,s,ne,s,n,n,s,sw,ne,ne,n,nw,n,ne,n,ne,ne,n,se,ne,n,ne,ne,n,ne,n,sw,n,n,n,se,ne,n,ne,ne,n,ne,nw,ne,sw,n,ne,nw,sw,n,n,ne,ne,n,ne,ne,ne,n,se,ne,ne,n,sw,ne,ne,se,ne,ne,ne,ne,ne,sw,ne,ne,ne,n,ne,sw,ne,nw,n,sw,ne,ne,n,ne,n,sw,n,ne,ne,nw,ne,ne,ne,n,n,n,ne,s,se,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,n,ne,ne,n,n,ne,s,se,ne,ne,ne,s,ne,ne,ne,n,ne,ne,n,ne,ne,ne,ne,ne,ne,s,ne,se,ne,n,ne,n,ne,ne,ne,n,ne,ne,ne,s,ne,n,ne,ne,ne,n,ne,ne,ne,se,ne,ne,ne,ne,ne,n,ne,s,n,ne,ne,se,s,ne,ne,ne,ne,ne,ne,nw,sw,ne,ne,s,ne,nw,ne,ne,ne,n,n,ne,ne,ne,s,ne,n,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,s,ne,ne,ne,nw,ne,ne,ne,ne,ne,nw,n,sw,ne,se,ne,n,ne,ne,nw,s,ne,ne,ne,ne,n,se,ne,ne,n,ne,ne,s,ne,ne,s,ne,ne,ne,ne,nw,ne,ne,ne,ne,ne,ne,ne,nw,se,se,ne,ne,ne,ne,ne,ne,sw,ne,s,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,se,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,se,se,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,sw,ne,n,se,ne,ne,ne,nw,ne,se,ne,ne,n,ne,ne,ne,se,sw,ne,ne,se,n,ne,s,ne,ne,ne,sw,ne,s,se,ne,nw,ne,s,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,se,ne,ne,ne,se,ne,ne,ne,sw,se,ne,ne,ne,ne,ne,se,ne,ne,ne,n,se,se,ne,ne,ne,se,ne,ne,ne,ne,se,se,ne,n,se,nw,ne,se,se,se,ne,se,n,ne,se,se,nw,se,ne,sw,se,se,ne,ne,ne,nw,se,ne,ne,ne,ne,ne,ne,ne,n,se,ne,ne,ne,nw,ne,se,ne,ne,se,ne,sw,ne,se,ne,ne,ne,se,se,ne,ne,se,se,ne,ne,se,ne,ne,ne,ne,nw,ne,ne,se,ne,ne,se,ne,se,ne,ne,ne,ne,ne,se,sw,ne,ne,ne,ne,se,se,ne,ne,ne,ne,ne,ne,se,ne,s,se,n,ne,ne,se,se,se,se,ne,s,ne,ne,se,se,s,ne,nw,ne,ne,ne,se,ne,ne,ne,ne,n,ne,ne,ne,se,ne,ne,ne,se,se,ne,ne,se,ne,ne,ne,ne,se,sw,nw,se,ne,ne,se,ne,s,se,sw,ne,se,ne,nw,ne,ne,s,se,ne,se,sw,ne,s,ne,ne,sw,se,se,ne,ne,se,ne,se,ne,ne,s,se,se,ne,se,ne,nw,se,sw,ne,ne,se,n,se,ne,ne,se,se,se,ne,se,se,se,ne,se,ne,se,ne,se,ne,s,n,ne,se,se,ne,sw,se,ne,se,ne,se,sw,se,nw,se,se,se,ne,se,se,se,ne,se,se,se,ne,nw,se,nw,se,nw,se,se,nw,se,s,ne,se,ne,n,se,ne,se,ne,se,se,ne,se,ne,ne,se,ne,ne,se,se,ne,se,ne,se,se,se,se,ne,nw,se,se,se,ne,se,sw,ne,se,nw,s,ne,se,se,ne,se,ne,se,se,se,sw,ne,se,ne,ne,se,s,ne,se,se,sw,se,ne,se,ne,ne,se,nw,se,se,ne,se,se,n,ne,ne,s,sw,se,se,se,ne,se,n,nw,se,ne,n,se,se,ne,se,se,se,se,se,nw,se,nw,se,se,ne,ne,n,ne,ne,se,se,sw,se,s,ne,se,ne,se,se,ne,ne,ne,ne,se,se,nw,se,se,ne,se,se,ne,se,se,se,ne,sw,se,ne,ne,nw,se,n,ne,se,se,ne,se,ne,n,se,se,se,ne,ne,se,nw,s,nw,se,se,s,s,ne,ne,ne,se,se,se,se,ne,se,ne,se,ne,se,sw,se,ne,se,ne,sw,se,se,se,ne,se,n,se,ne,ne,ne,se,nw,se,se,se,se,se,ne,se,ne,ne,ne,ne,ne,se,n,se,se,n,se,se,se,se,se,ne,ne,se,nw,se,ne,s,se,s,se,ne,n,s,se,se,se,ne,se,se,n,se,se,se,nw,ne,ne,ne,sw,se,ne,nw,se,se,ne,se,se,se,sw,se,se,se,n,nw,nw,se,n,sw,sw,se,ne,n,nw,n,se,se,se,s,ne,ne,se,se,se,se,ne,se,sw,se,se,se,se,s,ne,se,s,se,se,n,se,ne,se,ne,se,nw,se,se,nw,se,se,se,se,se,se,se,ne,se,se,se,se,se,se,se,se,se,ne,se,se,se,se,se,nw,se,se,se,se,n,se,se,se,se,n,s,se,se,se,se,ne,ne,se,n,se,n,sw,ne,s,se,ne,n,se,se,se,se,se,sw,se,se,se,ne,se,se,sw,se,se,s,se,ne,n,nw,se,se,ne,se,se,se,se,se,se,se,ne,sw,se,se,se,se,se,se,se,n,se,ne,nw,se,ne,se,se,se,nw,ne,n,se,se,se,se,se,se,ne,se,se,sw,s,se,se,se,se,se,ne,se,se,se,se,ne,se,s,se,se,nw,se,se,se,se,se,se,ne,se,n,nw,se,se,se,se,se,se,n,se,nw,nw,n,ne,se,se,nw,se,nw,s,se,s,se,se,sw,se,se,se,se,se,s,se,se,se,nw,se,n,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,nw,se,se,n,se,se,se,se,se,ne,se,se,se,se,se,se,s,se,ne,se,se,se,se,se,nw,se,se,se,se,se,se,se,se,sw,se,s,se,se,se,se,se,n,se,s,s,se,se,s,s,se,se,s,se,n,ne,nw,s,se,se,sw,se,ne,se,s,se,se,ne,se,se,sw,se,se,n,se,nw,s,se,se,se,s,se,sw,s,s,se,s,se,se,se,se,nw,se,se,se,s,se,se,s,se,s,ne,se,se,se,se,n,se,se,se,s,se,n,se,se,s,se,se,s,se,se,s,se,se,s,se,se,se,se,se,nw,se,nw,se,se,ne,se,s,sw,se,s,s,se,se,se,se,se,se,se,se,se,se,se,sw,se,se,se,se,s,se,se,se,s,ne,n,se,se,se,sw,se,se,n,s,se,se,se,se,se,se,se,ne,sw,s,s,s,se,nw,se,se,sw,se,s,s,se,se,nw,nw,se,nw,nw,s,se,n,ne,se,sw,se,se,se,ne,s,se,se,se,s,se,se,se,s,se,n,se,sw,se,se,se,se,se,se,s,se,se,sw,se,s,se,n,se,se,se,se,se,n,ne,s,s,se,nw,se,se,se,se,se,se,se,se,s,se,s,n,se,se,s,s,ne,sw,s,se,se,s,s,se,s,se,se,se,se,se,se,se,s,s,s,s,se,s,se,se,s,se,se,se,s,s,s,se,se,nw,se,ne,ne,se,nw,ne,se,ne,se,se,se,se,se,se,se,sw,sw,ne,se,s,se,se,s,s,s,s,s,s,nw,se,s,s,se,s,se,se,se,se,s,s,se,s,s,sw,s,s,se,n,se,s,se,n,s,se,se,se,se,ne,se,se,se,s,se,se,ne,s,s,ne,se,sw,sw,se,se,se,se,s,se,nw,s,s,s,s,se,se,se,se,sw,se,se,se,s,se,se,s,s,se,se,se,se,s,s,se,se,s,se,s,s,se,se,se,s,s,s,s,se,n,se,se,se,s,ne,s,n,se,s,ne,se,sw,se,se,sw,sw,s,s,nw,s,se,sw,se,nw,s,se,s,se,s,s,ne,se,n,se,s,se,nw,se,sw,s,se,s,se,s,se,se,se,s,s,sw,sw,s,ne,s,se,sw,se,se,s,s,s,n,se,se,s,s,s,nw,se,se,nw,s,s,s,s,n,s,s,s,n,se,se,se,s,s,se,s,se,s,n,n,sw,se,s,s,se,se,s,n,s,se,se,n,n,s,s,se,se,s,se,se,n,s,s,n,sw,ne,s,sw,s,se,s,nw,se,n,nw,s,s,s,se,ne,s,s,ne,s,sw,s,s,s,s,s,se,s,se,se,se,s,s,sw,s,s,se,se,se,s,se,s,ne,s,n,s,s,nw,se,s,s,s,nw,se,s,se,ne,s,s,ne,se,nw,se,s,n,s,n,se,se,se,s,s,sw,s,s,s,se,se,s,s,n,se,se,nw,n,n,s,s,s,ne,se,s,s,s,s,ne,n,s,s,s,s,se,se,se,se,se,s,nw,s,s,s,n,se,s,s,s,se,se,s,s,s,nw,s,s,se,se,s,s,s,s,s,s,s,s,sw,s,s,s,s,se,s,se,se,s,s,se,s,nw,se,s,ne,s,s,se,s,ne,se,s,s,ne,se,n,s,nw,se,ne,se,se,se,se,se,n,n,se,n,se,s,nw,s,s,s,s,s,ne,s,sw,s,s,s,se,s,nw,s,s,se,nw,s,se,s,s,se,s,se,sw,se,n,s,se,s,s,s,s,n,ne,s,se,n,s,se,s,se,se,n,s,se,s,s,s,sw,s,s,s,se,s,se,s,ne,se,nw,s,nw,s,sw,s,s,s,s,n,se,s,n,se,s,s,s,s,s,s,se,s,s,n,s,se,s,s,ne,s,s,se,se,se,se,s,s,s,se,s,s,n,s,s,s,s,se,se,n,n,s,se,s,s,s,s,n,s,s,se,se,s,s,s,s,s,n,n,ne,s,s,s,s,nw,s,s,s,s,s,s,n,s,s,se,s,se,s,s,se,s,s,s,sw,ne,s,sw,s,s,se,s,se,s,s,s,se,s,s,s,s,s,s,s,se,n,s,s,s,se,n,s,s,se,nw,s,s,n,s,s,ne,nw,s,s,s,se,s,s,s,s,s,s,se,s,s,s,se,se,se,s,s,s,s,s,se,nw,s,s,s,se,s,nw,s,s,s,s,s,s,ne,s,s,se,n,s,s,ne,s,s,s,s,s,s,s,s,se,s,s,s,sw,s,s,s,s,s,s,n,s,se,se,ne,s,s,s,s,ne,s,sw,s,s,sw,s,sw,s,s,s,s,s,s,ne,s,se,s,s,s,s,s,sw,s,s,s,s,s,s,s,se,n,s,se,s,s,s,s,s,s,s,s,s,s,s,nw,s,se,s,n,s,s,s,s,s,sw,nw,s,s,ne,n,n,s,s,n,s,s,s,s,s,s,s,s,se,sw,n,s,s,sw,s,s,s,s,s,ne,s,s,nw,s,s,ne,s,s,s,s,s,s,s,s,sw,s,s,s,s,s,s,s,s,sw,sw,s,s,s,s,s,s,sw,s,sw,s,s,s,ne,s,s,s,s,s,sw,s,s,sw,se,nw,s,s,s,s,s,s,s,s,se,sw,s,s,s,s,s,s,s,s,s,s,s,sw,s,s,s,s,s,s,s,s,s,s,s,nw,sw,s,s,ne,s,s,s,s,s,s,s,s,nw,nw,s,sw,sw,s,s,s,se,s,se,s,n,sw,s,ne,s,sw,s,n,s,s,n,s,s,s,s,s,s,s,s,sw,sw,s,s,s,s,sw,s,nw,s,s,se,sw,nw,s,s,s,n,s,s,s,sw,sw,s,s,s,nw,s,s,sw,s,s,sw,sw,s,s,s,nw,s,s,s,se,s,s,s,se,s,n,s,s,s,s,s,n,s,sw,s,s,s,s,s,sw,s,s,s,n,s,s,nw,s,se,ne,s,sw,s,nw,n,s,s,n,s,s,s,s,s,s,sw,s,s,s,sw,s,s,s,s,s,s,s,sw,sw,ne,s,s,n,s,ne,s,s,s,s,s,s,sw,s,sw,n,ne,s,sw,s,s,s,ne,s,s,s,s,s,s,ne,s,s,s,s,s,s,s,s,sw,s,s,s,ne,s,s,sw,nw,s,sw,s,sw,nw,s,s,s,s,s,se,s,n,s,sw,s,s,s,nw,sw,se,se,s,sw,s,sw,s,sw,s,sw,s,sw,s,sw,s,n,sw,s,nw,s,s,sw,s,s,sw,s,s,s,s,sw,s,s,s,s,se,se,s,n,n,n,sw,ne,s,s,s,s,sw,s,n,s,s,s,sw,s,s,ne,s,s,s,s,ne,s,s,s,s,se,s,s,s,nw,se,nw,s,s,s,s,sw,sw,s,s,s,sw,sw,sw,s,se,s,n,sw,s,sw,s,sw,n,s,s,sw,s,s,s,s,s,s,nw,s,s,s,se,s,s,se,sw,sw,n,sw,sw,n,sw,sw,s,s,sw,s,n,sw,s,s,sw,se,s,s,s,sw,s,s,s,s,s,s,sw,ne,sw,sw,s,s,se,sw,s,n,s,s,s,s,nw,s,s,s,s,sw,s,s,s,ne,sw,sw,se,s,s,s,s,sw,n,s,s,s,s,s,s,s,s,s,sw,s,sw,s,s,sw,s,n,sw,s,se,ne,s,s,s,s,sw,se,s,s,s,s,s,s,s,s,s,sw,s,s,s,sw,sw,s,s,s,s,sw,se,s,s,s,s,s,s,s,s,s,s,s,s,nw,nw,sw,nw,s,s,s,sw,s,sw,n,s,s,ne,sw,s,s,s,s,sw,s,nw,sw,sw,sw,sw,sw,sw,sw,s,sw,n,sw,s,s,sw,se,sw,s,s,s,se,se,sw,s,sw,s,se,s,s,s,nw,s,n,sw,s,s,sw,sw,sw,s,s,n,sw,s,sw,se,sw,sw,sw,ne,s,sw,sw,nw,s,se,ne,s,s,s,sw,sw,s,s,sw,s,sw,s,sw,s,s,s,nw,sw,s,n,sw,ne,sw,sw,sw,s,sw,s,se,ne,sw,s,sw,n,n,s,sw,ne,n,ne,sw,s,s,sw,s,nw,sw,sw,s,sw,ne,ne,s,sw,sw,s,sw,sw,s,ne,se,s,sw,se,nw,n,nw,ne,se,s,s,ne,n,ne,s,s,sw,nw,se,s,s,s,s,s,sw,s,ne,s,n,sw,s,n,sw,s,s,s,se,s,s,sw,s,sw,sw,s,s,sw,s,sw,s,s,s,s,s,sw,sw,sw,sw,s,s,s,s,se,ne,sw,s,ne,s,s,ne,s,sw,sw,sw,ne,sw,sw,s,s,nw,sw,sw,sw,s,nw,sw,sw,s,sw,sw,s,n,sw,ne,s,sw,s,s,n,sw,ne,sw,nw,sw,s,ne,s,s,n,s,sw,s,se,sw,s,s,sw,s,sw,s,s,nw,nw,sw,s,s,s,se,s,sw,sw,sw,n,s,nw,sw,n,s,s,s,s,sw,sw,s,s,sw,s,s,sw,se,sw,sw,s,s,s,sw,sw,nw,sw,se,s,sw,se,se,sw,sw,s,sw,s,s,ne,s,sw,sw,sw,sw,sw,sw,s,sw,s,sw,sw,s,sw,n,sw,sw,nw,sw,sw,s,s,s,se,nw,s,n,sw,s,s,se,sw,s,sw,sw,sw,sw,s,s,sw,s,sw,s,s,s,sw,s,s,nw,n,sw,sw,se,sw,ne,s,s,se,s,sw,s,sw,sw,s,s,ne,sw,sw,sw,ne,se,sw,sw,sw,n,s,s,sw,s,s,sw,sw,sw,s,s,sw,sw,n,sw,s,s,sw,sw,s,sw,sw,sw,s,n,ne,sw,ne,sw,sw,ne,sw,ne,s,sw,s,sw,s,sw,sw,n,sw,sw,s,se,sw,sw,ne,sw,s,sw,s,sw,se,s,s,se,s,sw,sw,s,sw,s,s,s,sw,sw,sw,s,s,s,sw,sw,s,sw,s,ne,nw,sw,sw,sw,s,sw,s,s,sw,sw,se,ne,s,sw,s,sw,sw,se,sw,s,sw,ne,sw,s,se,s,sw,s,s,sw,s,sw,s,sw,s,sw,sw,s,s,sw,s,sw,nw,ne,sw,s,sw,sw,n,ne,sw,se,sw,s,sw,n,sw,s,se,s,se,se,sw,sw,n,sw,sw,sw,sw,sw,s,sw,sw,sw,nw,s,n,se,sw,sw,sw,n,sw,sw,sw,n,sw,sw,sw,sw,sw,ne,sw,s,n,s,sw,sw,sw,nw,s,sw,n,s,s,s,s,n,n,sw,s,s,n,sw,sw,se,nw,sw,sw,n,sw,s,sw,sw,sw,sw,sw,nw,s,s,sw,n,s,s,sw,s,s,ne,s,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,se,s,s,s,n,sw,s,sw,sw,sw,sw,nw,s,s,se,sw,n,sw,sw,nw,sw,sw,sw,sw,sw,s,sw,s,s,sw,sw,s,s,s,s,sw,sw,sw,sw,sw,s,sw,sw,n,sw,s,sw,se,sw,sw,n,s,s,s,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,s,sw,sw,s,s,s,ne,sw,sw,ne,sw,sw,sw,s,sw,sw,sw,sw,n,sw,s,ne,sw,sw,s,sw,sw,sw,s,sw,s,s,sw,sw,s,nw,se,s,sw,s,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,s,nw,s,sw,sw,sw,s,sw,sw,s,sw,n,sw,sw,nw,sw,sw,se,n,sw,sw,s,sw,s,sw,sw,ne,sw,sw,n,sw,se,sw,sw,s,sw,sw,sw,s,sw,sw,se,sw,sw,s,sw,sw,s,sw,sw,sw,sw,s,sw,sw,sw,sw,se,sw,s,sw,sw,sw,ne,sw,sw,sw,sw,sw,sw,sw,sw,s,s,ne,sw,s,s,sw,sw,s,se,sw,sw,sw,n,s,se,sw,sw,s,sw,s,s,ne,s,sw,sw,s,sw,s,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,nw,s,sw,sw,sw,nw,sw,sw,s,sw,se,s,sw,nw,n,s,sw,sw,nw,sw,sw,s,sw,ne,sw,sw,nw,sw,sw,sw,sw,ne,sw,ne,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,nw,sw,s,ne,sw,sw,sw,sw,sw,sw,se,s,sw,s,nw,sw,sw,sw,sw,se,sw,sw,se,sw,sw,ne,ne,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,s,sw,sw,ne,sw,sw,sw,sw,sw,sw,sw,s,ne,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,ne,n,sw,sw,sw,sw,n,sw,sw,se,s,se,s,nw,s,sw,ne,se,sw,ne,sw,sw,se,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,s,ne,sw,sw,sw,ne,sw,sw,sw,ne,sw,sw,sw,sw,nw,nw,s,sw,sw,sw,nw,sw,s,sw,s,sw,sw,sw,sw,n,sw,sw,ne,sw,sw,sw,sw,sw,s,s,sw,sw,sw,se,sw,sw,s,n,se,ne,ne,ne,ne,ne,n,n,n,nw,ne,sw,n,n,n,sw,nw,nw,nw,nw,nw,nw,n,sw,nw,nw,sw,sw,sw,nw,sw,sw,sw,nw,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,sw,s,s,sw,sw,s,s,s,s,ne,s,s,nw,s,s,s,s,s,sw,s,s,s,s,ne,se,s,se,s,se,se,se,se,s,se,n,s,se,s,s,s,se,se,se,se,s,n,se,se,se,se,nw,se,nw,se,ne,se,se,se,se,se,se,se,nw,sw,se,se,se,ne,se,se,se,n,n,ne,se,ne,n,se,n,se,ne,ne,se,se,nw,ne,ne,ne,ne,ne,se,ne,se,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,s,nw,ne,ne,n,ne,ne,ne,ne,s,sw,sw,ne,ne,ne,nw,n,ne,ne,n,s,n,ne,ne,ne,ne,se,s,n,ne,ne,ne,ne,ne,s,ne,n,n,ne,se,n,n,n,n,n,ne,n,n,n,n,sw,n,n,n,ne,nw,s,n,ne,n,n,n,se,s,sw,n,n,n,n,n,ne,n,n,n,sw,n,sw,n,n,n,n,n,n,n,sw,n,s,n,n,n,n,n,n,n,n,s,n,n,nw,se,n,n,se,n,n,n,nw,nw,n,n,nw,n,s,s,n,nw,n,ne,nw,n,n,n,nw,n,nw,sw,nw,n,nw,n,nw,n,n,nw,n,nw,s,n,n,n,nw,nw,nw,n,nw,n,nw,se,nw,nw,nw,s,n,nw,nw,nw,nw,se,ne,n,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,nw,nw,se,nw,nw,s,nw,nw,ne,sw,nw,nw,nw,nw,nw,ne,sw,nw,s,nw,nw,nw,nw,nw,nw,nw,se,nw,ne,nw,sw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,ne,nw,sw,nw,nw,sw,nw,nw,sw,sw,nw,sw,nw,n,sw,nw,sw,nw,n,ne,sw,sw,nw,nw,nw,sw,nw,nw,sw,nw,sw,sw,nw,sw,sw,nw,nw,nw,sw,sw,sw,sw,ne,sw,sw,s,sw,sw,nw,nw,sw,nw,sw,nw,nw,sw,ne,ne,sw,sw,s,sw,sw,sw,sw,s,nw,nw,nw,nw,sw,sw,n,sw,sw,sw,nw,sw,sw,sw,se,sw,n,s,n,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,n,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,sw,sw,se,ne,sw,sw,sw,sw,sw,sw,sw,s,se,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,s,sw,sw,s,sw,ne,sw,s,se,sw,ne,sw,sw,s,sw,sw,s,s,sw,s,sw,nw,n,sw,n,n,s,ne,s,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,s,s,sw,s,sw,sw,sw,sw,nw,sw,se,s,nw,n,s,s,sw,s,sw,s,s,n,se,sw,sw,s,sw,s,s,sw,s,s,sw,s,sw,sw,s,s,sw,s,n,s,sw,s,s,s,se,sw,s,s,sw,sw,n,sw,sw,sw,sw,s,nw,se,nw,s,s,s,sw,sw,sw,s,s,sw,s,sw,s,sw,s,sw,s,s,n,sw,s,s,sw,s,s,s,sw,s,s,s,s,sw,s,s,sw,s,nw,s,s,s,s,s,s,ne,n,n,s,s,s,nw,s,n,s,s,s,s,s,s,sw,s,s,sw,s,s,s,s,s,s,se,se,s,s,s,s,s,s,s,s,s,s,s,s,s,n,s,s,se,sw,s,s,sw,s,s,n,s,s,s,n,se,s,n,s,s,se,s,n,s,s,s,s,s,s,s,s,s,s,s,sw,s,se,s,s,s,se,s,se,s,se,s,se,se,se,s,s,ne,s,s,s,s,s,s,nw,s,s,s,s,s,s,s,s,s,s,s,s,s,s,ne,ne,s,s,s,nw,s,s,sw,s,sw,se,s,s,s,se,ne,s,s,n,s,se,s,s,s,s,sw,s,se,se,se,s,se,s,nw,ne,se,nw,s,s,ne,se,nw,s,s,nw,s,se,se,s,se,s,s,s,se,s,s,se,se,s,se,nw,s,se,s,sw,s,se,s,n,se,s,ne,se,s,ne,se,se,se,s,s,se,s,s,se,s,n,s,se,s,se,se,se,se,s,se,se,se,ne,se,nw,se,se,se,s,s,se,se,s,sw,se,s,s,s,ne,s,ne,ne,se,se,se,s,se,se,se,nw,sw,se,se,se,se,se,se,s,se,se,se,n,n,se,sw,se,nw,se,s,s,nw,se,se,s,se,se,s,sw,se,s,se,se,se,se,se,se,nw,se,s,ne,se,s,se,ne,n,se,sw,se,se,nw,se,se,se,se,s,sw,se,se,se,se,se,se,se,se,se,s,se,sw,se,se,se,se,s,se,se,sw,se,se,se,se,se,nw,se,se,se,se,sw,se,se,nw,se,sw,se,sw,se,sw,se,sw,se,se,se,se,se,se,s,se,se,nw,se,se,se,se,se,ne,se,se,se,ne,se,se,ne,se,s,ne,se,se,se,ne,se,se,se,se,se,se,se,se,se,n,ne,sw,se,se,ne,se,se,se,se,s,se,se,se,se,se,ne,ne,s,se,se,se,se,se,se,s,sw,se,se,n,se,se,se,se,se,se,ne,n,nw,se,n,se,ne,se,se,ne,se,s,ne,se,se,se,se,se,s,s,ne,se,se,se,ne,se,se,se,se,n,se,se,nw,se,se,s,s,se,se,se,se,nw,se,n,se,nw,se,ne,se,se,se,ne,ne,n,sw,se,ne,ne,se,se,nw,se,ne,ne,se,se,se,sw,se,se,ne,ne,ne,se,ne,ne,ne,se,nw,ne,ne,s,ne,ne,sw,se,se,ne,se,se,ne,se,s,n,se,sw,se,ne,se,se,ne,s,ne,se,ne,ne,se,ne,ne,ne,ne,se,ne,se,s,ne,nw,ne,nw,ne,ne,ne,ne,ne,ne,s,ne,se,ne,ne,se,ne,ne,ne,ne,n,ne,sw,se,ne,s,ne,s,s,ne,nw,se,se,se,se,sw,se,s,se,ne,ne,se,se,n,se,se,sw,se,ne,nw,nw,se,se,nw,ne,se,ne,se,ne,ne,ne,ne,nw,ne,s,n,ne,ne,ne,ne,s,ne,ne,se,ne,ne,s,se,sw,ne,ne,ne,n,s,ne,se,nw,ne,ne,se,sw,nw,ne,ne,s,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,n,ne,ne,s,se,se,ne,sw,ne,ne,ne,nw,ne,ne,s,sw,nw,ne,ne,ne,ne,ne,ne,ne,se,s,ne,ne,ne,se,ne,sw,ne,ne,se,ne,ne,sw,ne,nw,nw,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,se,ne,ne,sw,ne,ne,ne,ne,se,ne,ne,ne,nw,ne,ne,ne,ne,ne,n,ne,nw,ne,ne,ne,s,ne,ne,ne,ne,sw,ne,s,se,ne,ne,s,n,ne,ne,ne,sw,ne,ne,ne,ne,nw,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,sw,ne,nw,ne,se,se,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,n,sw,ne,ne,ne,ne,ne,ne,ne,s,ne,ne,se,s,se,ne,sw,se,s,nw,ne,ne,ne,ne,se,ne,ne,ne,n,ne,n,n,ne,n,se,ne,sw,ne,ne,ne,ne,ne,n,se,ne,se,n,s,n,nw,ne,n,s,ne,ne,ne,ne,ne,n,ne,ne,nw,ne,ne,s,se,ne,n,ne,se,n,ne,ne,n,nw,sw,n,n,ne,ne,ne,s,ne,ne,nw,ne,n,nw,ne,s,n,ne,nw,n,se,n,ne,ne,se,ne,n,ne,ne,ne,nw,s,sw,ne,ne,sw,n,ne,ne,ne,n,sw,ne,ne,se,ne,n,n,ne,ne,se,nw,ne,ne,ne,ne,se,n,se,s,ne,ne,nw,se,n,nw,n,nw,ne,ne,ne,ne,ne,sw,ne,n,ne,n,ne,ne,se,n,se,ne,ne,ne,ne,n,ne,ne,ne,s,ne,ne,n,n,sw,ne,n,ne,ne,ne,n,se,nw,ne,n,ne,n,se,n,n,ne,n,ne,ne,ne,ne,n,n,ne,ne,se,ne,ne,n,n,ne,n,nw,ne,n,ne,ne,ne,ne,ne,n,ne,n,nw,ne,ne,s,n,n,ne,ne,n,nw,ne,n,sw,sw,ne,nw,ne,nw,n,n,n,n,se,n,sw,n,ne,ne,ne,nw,ne,n,s,ne,ne,n,ne,n,n,n,n,n,n,n,ne,ne,n,ne,n,n,n,n,n,ne,ne,ne,se,n,ne,n,s,n,se,n,ne,n,ne,sw,n,n,n,n,n,n,s,ne,n,n,se,n,nw,n,n,n,n,ne,s,n,n,n,n,ne,ne,ne,sw,n,n,n,n,ne,n,n,ne,nw,n,ne,n,ne,n,n,ne,ne,n,ne,se,n,ne,nw,ne,ne,sw,ne,ne,nw,ne,ne,ne,nw,n,sw,se,nw,n,n,sw,n,sw,sw,n,n,n,s,sw,s,n,se,ne,n,ne,n,n,n,n,ne,n,n,n,n,sw,sw,ne,ne,se,sw,s,n,n,n,ne,n,n,n,n,n,nw,n,n,n,ne,n,s,n,n,n,nw,n,n,ne,n,n,ne,n,sw,n,n,n,n,n,ne,n,nw,n,n,n,n,n,n,s,nw,n,ne,n,n,n,ne,n,n,ne,n,n,s,ne,ne,n,se,n,n,ne,se,n,n,n,n,ne,n,ne,n,n,n,n,se,n,ne,sw,n,s,s,n,sw,n,n,sw,nw,n,nw,n,n,n,n,n,n,n,nw,nw,n,sw,n,n,n,n,n,ne,n,s,n,n,s,n,n,n,ne,s,s,n,s,se,n,n,n,n,n,n,se,nw,sw,n,n,n,n,n,n,n,n,n,n,n,nw,sw,n,n,ne,n,n,n,n,n,n,n,n,n,n,n,n,se,s,n,s,n,s,n,nw,n,n,ne,n,ne,s,n,n,s,n,ne,ne,n,n,n,n,nw,ne,n,se,sw,n,n,sw,n,n,n,se,n,n,se,n,se,n,n,s,n,n,n,nw,ne,n,n,n,n,n,n,n,s,n,nw,n,n,n,se,n,n,n,n,n,n,n,n,n,s,n,n,n,se,sw,n,n,sw,n,s,nw,n,nw,n,n,se,n,sw,n,n,n,n,n,nw,n,n,n,n,s,n,n,n,n,nw,nw,n,n,n,se,nw,n,nw,se,n,n,n,n,ne,n,n,n,s,se,sw,sw,nw,s,nw,ne,n,n,nw,n,nw,n,n,ne,n,n,n,n,n,n,se,se,n,se,n,nw,n,n,n,n,nw,nw,nw,n,s,n,nw,n,n,n,ne,n,n,n,nw,nw,n,n,s,n,nw,n,n,n,n,n,nw,n,nw,n,n,n,s,se,n,n,n,nw,s,n,s,n,n,n,n,n,n,se,n,n,n,n,n,sw,n,nw,n,n,n,nw,n,n,se,s,n,nw,nw,n,n,n,n,se,n,n,n,n,ne,sw,nw,nw,se,n,nw,nw,nw,s,se,ne,nw,n,s,n,n,s,n,s,s,n,nw,n,ne,nw,n,se,n,nw,n,se,nw,nw,n,nw,n,s,nw,nw,nw,n,n,nw,n,ne,nw,se,n,n,n,nw,nw,n,s,n,n,nw,n,nw,s,nw,sw,nw,n,nw,se,n,n,n,ne,n,nw,n,ne,nw,n,nw,n,n,se,n,nw,nw,n,se,nw,se,se,nw,n,nw,nw,n,ne,sw,se,n,n,nw,s,n,nw,nw,n,nw,nw,n,nw,nw,nw,s,nw,n,se,nw,n,n,s,s,n,n,n,ne,n,nw,n,n,n,n,sw,n,n,se,ne,n,nw,nw,n,n,nw,sw,nw,ne,n,nw,n,n,se,se,n,n,nw,n,se,n,s,nw,n,n,n,n,nw,n,n,n,ne,nw,nw,nw,n,nw,n,n,n,nw,nw,nw,n,ne,nw,n,n,sw,n,n,n,nw,nw,s,n,nw,nw,sw,n,n,nw,n,n,n,n,nw,n,n,n,nw,nw,nw,nw,n,s,nw,nw,n,nw,se,n,nw,nw,n,n,nw,n,nw,n,nw,n,n,nw,n,nw,n,n,n,n,nw,nw,nw,nw,nw,n,n,n,sw,n,n,nw,nw,n,n,n,n,ne,n,sw,n,n,nw,n,nw,nw,nw,nw,nw,nw,nw,ne,nw,ne,n,n,nw,nw,nw,nw,nw,nw,nw,nw,se,nw,nw,nw,n,nw,ne,nw,se,nw,nw,nw,n,se,nw,n,se,nw,nw,nw,ne,n,n,sw,nw,n,ne,ne,nw,n,nw,sw,n,n,nw,nw,n,ne,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,s,nw,nw,nw,n,nw,nw,sw,n,se,nw,nw,nw,nw,n,nw,s,n,nw,nw,n,nw,nw,nw,sw,nw,n,nw,nw,sw,n,nw,nw,nw,s,nw,nw,nw,nw,n,ne,nw,nw,nw,se,n,n,nw,nw,nw,n,nw,nw,s,nw,nw,n,nw,nw,nw,sw,se,nw,nw,nw,nw,n,s,nw,nw,nw,nw,ne,se,sw,nw,sw,nw,nw,n,n,n,nw,n,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,se,se,se,n,sw,nw,s,ne,n,nw,nw,s,n,nw,s,n,ne,s,se,nw,nw,n,nw,nw,ne,nw,nw,nw,nw,nw,nw,ne,nw,n,nw,nw,nw,nw,n,nw,s,n,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,ne,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,sw,nw,sw,nw,n,ne,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,se,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,se,nw,nw,nw,n,nw,ne,nw,nw,ne,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,ne"#;
