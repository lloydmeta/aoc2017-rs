use std::fmt;

const OPEN_GROUP: char = '{';
const CLOSE_GROUP: char = '}';
const OPEN_GARBAGE: char = '<';
const CLOSE_GARBAGE: char = '>';
const SKIP_NEXT_GARBAGE: char = '!';

const DAY_9_INPUT: &'static str = include_str!("../data/day_9_input");

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 9: Stream Processing ***");
    println!("Input: {}", DAY_9_INPUT);
    println!("Solution: {}\n", count_groups(DAY_9_INPUT));
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct StreamStats {
    sums: Vec<usize>, // for debugging
    total_groups: usize,
    total_garbage: usize,
}

impl fmt::Display for StreamStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StreamStats {{ total_groups: {}, total_garbage: {} }}",
            self.total_groups, self.total_garbage
        )
    }
}

fn count_groups(s: &str) -> StreamStats {
    let mut current_level = 0;
    let mut ignore_next = false;
    let mut in_garbage = false;

    s.chars().fold(
        StreamStats {
            sums: Vec::with_capacity(s.len() / 2),
            total_groups: 0,
            total_garbage: 0,
        },
        |mut acc, next| {
            if in_garbage {
                if ignore_next {
                    // cancel
                    ignore_next = false;
                } else if next == CLOSE_GARBAGE {
                    in_garbage = false;
                    ignore_next = false; // just in case
                } else if next == SKIP_NEXT_GARBAGE {
                    ignore_next = true;
                } else {
                    acc.total_garbage += 1;
                }
            } else {
                if next == OPEN_GROUP {
                    current_level += 1;
                } else if next == CLOSE_GROUP {
                    if current_level != 0 {
                        acc.sums.push(current_level);
                        acc.total_groups += current_level;
                        current_level -= 1;
                    }
                } else if next == OPEN_GARBAGE {
                    in_garbage = true;
                    ignore_next = false; // just in case
                }
            }
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use day_9::*;

    #[test]
    fn count_groups_test() {
        assert_eq!(count_groups("{}").total_groups, 1);
        assert_eq!(count_groups("{{}}").total_groups, 3);
        assert_eq!(count_groups("{{{}}}").total_groups, 6);
        assert_eq!(count_groups("{{},{}},").total_groups, 5);
        assert_eq!(count_groups("{{{},{},{{}}}},").total_groups, 16);
        assert_eq!(count_groups("{<a>,<a>,<a>,<a>},").total_groups, 1);
        assert_eq!(
            count_groups("{{<ab>},{<ab>},{<ab>},{<ab>}},").total_groups,
            9
        );
        assert_eq!(
            count_groups("{{<!!>},{<!!>},{<!!>},{<!!>}},").total_groups,
            9
        );
        assert_eq!(
            count_groups("{{<a!>},{<a!>},{<a!>},{<ab>}},").total_groups,
            3
        );
    }

    #[test]
    fn count_groups_real_test() {
        let r = count_groups(DAY_9_INPUT);
        assert_eq!(r.total_groups, 21037);
        assert_eq!(r.total_garbage, 9495);
    }

}
