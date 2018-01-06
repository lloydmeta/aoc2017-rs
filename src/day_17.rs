const DAY_17_STEP_SIZE: usize = 348;
const DAY_17_STEPS: usize = 2017;
pub fn run() -> Result<(), &'static str> {
    println!("*** Day 17: Spinlock ***");
    println!(
        "Input steps:[{}] step_size:[{}]",
        DAY_17_STEPS, DAY_17_STEP_SIZE
    );
    println!(
        "Solution1: {:?}",
        after_target(DAY_17_STEPS, DAY_17_STEP_SIZE, 2017)
    );
    println!(
        "Solution2: {:?}",
        after_nth(50_000_000, DAY_17_STEP_SIZE, 0)
    );
    Ok(())
}

fn after_target(steps: usize, step_size: usize, target: usize) -> Option<usize> {
    let spin_lock = SpinLock {
        steps: steps,
        step_size: step_size,
    };
    let result = spin_lock.run();
    let r: Vec<_> = result
        .0
        .iter()
        .skip_while(|e| *e != &target)
        .skip(1)
        .collect();
    r.get(0).map(|u| **u)
}

fn after_nth(steps: usize, step_size: usize, nth: usize) -> Option<usize> {
    (0..steps)
        .into_iter()
        .fold((None, 0), |(acc, curr_idx), next| {
            let insert_at = SpinLock::insert_at(next + 1, step_size, curr_idx);
            if insert_at == nth + 1 {
                (Some(next + 1), insert_at)
            } else {
                (acc, insert_at)
            }
        })
        .0
}

#[derive(Debug)]
struct SpinLock {
    step_size: usize,
    steps: usize,
}

impl SpinLock {
    fn run(&self) -> SpinLockResult {
        let (r, _) = (0..self.steps).fold(
            (SpinLockResult(self.init_buffer()), 0),
            |(mut result, idx), next_step| {
                let next_value = next_step + 1;
                let next_idx = SpinLock::insert_at(result.0.len(), self.step_size, idx);
                result.0.insert(next_idx, next_value);
                (result, next_idx)
            },
        );
        r
    }

    fn init_buffer(&self) -> Vec<usize> {
        let mut v = Vec::with_capacity(self.steps);
        v.push(0);
        v
    }

    fn insert_at(curr_length: usize, step_increase: usize, current: usize) -> usize {
        (current + step_increase) % curr_length + 1
    }
}

#[derive(Debug)]
struct SpinLockResult(Vec<usize>);

#[cfg(test)]
mod tests {
    use day_17::*;

    #[test]
    fn after_target_test() {
        assert_eq!(
            after_target(DAY_17_STEPS, DAY_17_STEP_SIZE, 2017),
            Some(417)
        );
    }

    #[test]
    fn after_nth_test() {
        assert_eq!(after_nth(50_000_000, DAY_17_STEP_SIZE, 0), Some(34334221));
    }
}
