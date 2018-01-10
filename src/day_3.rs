const DAY_3_INPUT: u64 = 368078;

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 3: Spiral Memory ***");
    println!("Input: {}", DAY_3_INPUT);
    println!("Solution: {}\n", steps_to_centre(DAY_3_INPUT)?);
    Ok(())
}

fn steps_to_centre(idx: u64) -> Result<u64, &'static str> {
    let Coords { x, y } = idx_to_coords(idx)?;
    let total = x.abs() + y.abs();
    Ok(total as u64)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

fn idx_to_coords(idx: u64) -> Result<Coords, &'static str> {
    if idx > 0 {
        let r = {
            let idx_to_check = idx as i64;
            let layer = (((idx as f64).sqrt() - 1f64) / 2f64).ceil() as i64;
            let points_in_layer = 2 * layer + 1;
            // this is the greatest index in the layer
            let lower_right_idx = points_in_layer.pow(2);
            let side_width = points_in_layer - 1;
            if idx_to_check >= (lower_right_idx - side_width) {
                // we are on the south-side of the layer
                Coords {
                    x: layer - (lower_right_idx - idx_to_check),
                    y: -layer,
                }
            } else {
                let lower_left_idx = lower_right_idx - side_width;
                if idx_to_check >= (lower_left_idx - side_width) {
                    Coords {
                        x: -layer,
                        y: -layer + (lower_left_idx - idx_to_check),
                    }
                } else {
                    let upper_left_idx = lower_left_idx - side_width;
                    if idx_to_check >= upper_left_idx - side_width {
                        Coords {
                            x: -layer + (upper_left_idx - idx_to_check),
                            y: layer,
                        }
                    } else {
                        let upper_right_idx = upper_left_idx - side_width;
                        Coords {
                            x: layer,
                            y: layer - (upper_right_idx - idx_to_check),
                        }
                    }
                }
            }
        };
        Ok(r)
    } else {
        Err("Idx must be greater than 0")
    }
}

#[cfg(test)]
mod tests {
    use day_3::*;

    #[test]
    fn idx_to_coords_test() {
        assert_eq!(idx_to_coords(1).unwrap(), Coords { x: 0, y: 0 });
        assert_eq!(idx_to_coords(2).unwrap(), Coords { x: 1, y: 0 });
        assert_eq!(idx_to_coords(9).unwrap(), Coords { x: 1, y: -1 });
        assert_eq!(idx_to_coords(22).unwrap(), Coords { x: -1, y: -2 });
    }

}
