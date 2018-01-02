use common::*;

pub fn run() -> Result<(), String> {
    println!("*** Day 10: Knot Hash ***");
    println!("Input: {}", DAY_10_INPUT);
    println!("Solution1: {}\n", solve_knot_hash(DAY_10_INPUT)?);
    println!("Solution2: {}\n", hex_knot_hash(DAY_10_INPUT)?);
    Ok(())
}

fn solve_knot_hash(s: &str) -> Result<usize, String> {
    let v = knot_hash(s)?;
    if v.len() > 1 {
        Ok(v[0] * v[1])
    } else {
        Err(format!(
            "Result not long enough to find the answer to life: {:?}",
            v
        ))
    }
}

const DAY_10_INPUT: &'static str = "88,88,211,106,141,1,78,254,2,111,77,255,90,0,54,205";

#[cfg(test)]
mod tests {
    use day_10::*;

    #[test]
    fn solve_knot_hash_test() {
        let r = solve_knot_hash(DAY_10_INPUT).unwrap();
        assert_eq!(r, 11375);
    }

}
