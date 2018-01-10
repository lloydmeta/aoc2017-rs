use std::u64;
use std::collections::HashSet;

const DAY_6_INPUT: &str = include_str!("../data/day_6_input");

pub fn run() -> Result<(), String> {
    println!("*** Day 6: Memory Reallocation ***");
    println!("Input: {}", DAY_6_INPUT);
    let mut redistributer = RedistributionCycles::new(DAY_6_INPUT);
    println!("Solution 1: {:?}\n", redistributer.redist()?);
    println!("Solution 2: {:?}\n", redistributer.loop_size()?);
    Ok(())
}

struct RedistributionCycles {
    seen_configs: Vec<Vec<u64>>,
    // This is just an optimisation so we can look for seen configs faster than
    // looping through a Vector
    seen_configs_hash: HashSet<Vec<u64>>,
    current: Vec<u64>,
    cycles: u64,
}

#[derive(PartialEq, Eq, Debug)]
struct RepeatsAfter(u64);

#[derive(PartialEq, Eq, Debug)]
struct LoopCycle(u64);

impl RedistributionCycles {
    fn new(s: &str) -> RedistributionCycles {
        let init = s.split("\t")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        RedistributionCycles {
            seen_configs: Vec::new(),
            seen_configs_hash: HashSet::new(),
            current: init,
            cycles: 0,
        }
    }

    fn loop_size(&self) -> Result<LoopCycle, &str> {
        if self.cycles == 0 {
            Err("redist() not yet run.")
        } else {
            let pos = self.seen_configs.iter().position(|v| *v == self.current);
            match pos {
                Some(idx) => Ok(LoopCycle(self.cycles - idx as u64)),
                None => Err("No loop found"),
            }
        }
    }

    fn redist(&mut self) -> Result<RepeatsAfter, String> {
        while !self.seen_configs_hash.contains(&self.current) && self.cycles <= u64::max_value() {
            self.redist_once()
        }
        if !self.seen_configs_hash.contains(&self.current) {
            Err(format!(
                "Cycled {} times and couldn't find a repeat",
                self.cycles
            ))
        } else {
            Ok(RepeatsAfter(self.cycles))
        }
    }

    fn redist_once(&mut self) -> () {
        // Archive current form
        self.seen_configs.push(self.current.clone());
        self.seen_configs_hash.insert(self.current.clone());

        let mut idx = find_redist_target_idx(&self.current);
        let mut redis_load = self.current[idx];
        self.current[idx] = 0;
        while redis_load > 0 {
            if idx + 1 >= self.current.len() {
                idx = 0;
            } else {
                idx += 1;
            }
            self.current[idx] += 1;
            redis_load -= 1;
        }
        self.cycles += 1;
    }
}

fn find_redist_target_idx(v: &Vec<u64>) -> usize {
    v.iter().enumerate().fold(0, |acc, (idx, next)| {
        let last_biggest = v[acc];
        if *next > last_biggest {
            idx
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use day_6::*;

    #[test]
    fn find_redist_target_idx_test() {
        assert_eq!(find_redist_target_idx(&vec![0, 2, 7, 0]), 2);
        assert_eq!(find_redist_target_idx(&vec![3, 1, 2, 3]), 0);
    }

    #[test]
    fn find_repeat_test() {
        let mut runner = RedistributionCycles::new("0\t2\t7\t0");
        assert_eq!(runner.redist(), Ok(RepeatsAfter(5)));
    }
}
