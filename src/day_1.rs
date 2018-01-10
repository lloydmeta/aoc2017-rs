const RADIX: u32 = 10;

const DAY_1_INPUT: &str = include_str!("../data/day_1_input");

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 1: Inverse Captcha ***");
    println!("Input: {}", DAY_1_INPUT);
    println!("Solution: {}\n", sum_match_nexts(DAY_1_INPUT));
    Ok(())
}

fn string_to_digits(s: &str) -> Vec<u8> {
    s.chars()
        .filter_map(|c| c.to_digit(RADIX))
        .map(|d| d as u8)
        .collect()
}

fn sum_match_nexts(num_s: &str) -> u64 {
    fn go(nums: &Vec<u8>) -> u64 {
        let tail_with_first = {
            let mut t: Vec<_> = nums.into_iter().skip(1).map(|i| *i).collect();
            if nums.len() > 0 {
                t.push(nums[0]);
            }
            t
        };
        nums.into_iter()
            .zip(tail_with_first.iter())
            .filter(|&(i, j)| i == j)
            .fold(0, |acc, (v, _)| *v as u64 + acc)
    }
    let nums = string_to_digits(num_s);
    go(&nums)
}

#[cfg(test)]
mod tests {
    use day_1::*;

    #[test]
    fn string_to_digits_test() {
        assert_eq!(string_to_digits("1234"), vec![1, 2, 3, 4]);
    }

}
