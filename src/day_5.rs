const SPLIT_ON: &str = "\n";

pub fn steps_to_escape(inst_str: &str) -> Result<u64, &'static str> {
    let parsed = instructions_str_to_vec(inst_str);
    steps_to_escape_vec_inner(&parsed, |j| j + 1)
}

pub fn steps_to_escape_next(inst_str: &str) -> Result<u64, &'static str> {
    let parsed = instructions_str_to_vec(inst_str);
    steps_to_escape_vec_inner(&parsed, |j| if j >= 3 { j - 1 } else { j + 1 })
}

fn instructions_str_to_vec(inst_str: &str) -> Vec<i64> {
    inst_str
        .split(SPLIT_ON)
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn steps_to_escape_vec_inner<F>(
    instructions: &Vec<i64>,
    bump_jump_with: F,
) -> Result<u64, &'static str>
where
    F: Fn(i64) -> i64,
{
    let mut instructions_scratchpad = instructions.clone();
    let instructions_length = instructions.len();
    if instructions_length == 0 {
        Err("Can't escape a zero-length instructions list")
    } else {
        let mut steps_taken: u64 = 0;
        let mut current_idx: i64 = 0;
        let mut escaped = false;
        while !escaped && steps_taken <= u64::max_value() {
            steps_taken += 1;
            let next_jump = instructions_scratchpad[current_idx as usize];
            let jump_bump = bump_jump_with(next_jump);
            instructions_scratchpad[current_idx as usize] = jump_bump;
            current_idx = current_idx + next_jump;
            if current_idx < 0 || current_idx >= instructions_length as i64 {
                escaped = true;
            }
        }
        Ok(steps_taken)
    }
}

#[cfg(test)]
mod tests {
    use day_5::*;

    #[test]
    fn steps_to_escape_vec_test() {
        assert_eq!(
            steps_to_escape_vec_inner(&vec![0, 3, 0, 1, -3], |j| j + 1).unwrap(),
            5
        );
    }

}
