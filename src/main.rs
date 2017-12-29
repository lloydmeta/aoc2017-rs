extern crate aoc_2017;

use std::error::Error;
use std::process::exit;

use aoc_2017::day_1::*;
use aoc_2017::day_2::*;
use aoc_2017::day_3::*;
use aoc_2017::day_4::*;
use aoc_2017::day_5::*;
use aoc_2017::day_6::*;
use aoc_2017::day_7::*;
use aoc_2017::day_8::*;
use aoc_2017::day_9::*;
use aoc_2017::day_10::*;
use aoc_2017::day_11::*;

fn main() {
    match main_result() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Something went horribly wrong: {}", e);
            exit(1)
        }
    }
}

fn main_result() -> Result<(), Box<Error>> {
    println!("*** Day 1: Inverse Captcha ***");
    println!("Input: {}", DAY_1_INPUT);
    println!("Solution: {}\n", sum_match_nexts(DAY_1_INPUT));

    println!("*** Day 2: Corruption Checksum ***");

    println!("Input: {}", DAY_2_INPUT);
    println!("Solution: {}\n", checksum(DAY_2_INPUT));

    println!("*** Day 3: Spiral Memory ***");
    println!("Input: {}", DAY_3_INPUT);
    println!("Solution: {}\n", steps_to_centre(DAY_3_INPUT)?);

    println!("*** Day 4: High-Entropy Passphrases ***");
    println!("Input: {}", DAY_4_INPUT);
    let passphrases: Vec<_> = DAY_4_INPUT.trim().split("\n").collect();
    let valid_passphrases_1 = passphrases
        .iter()
        .filter(|s| are_valid_passphrases(s))
        .count();
    let valid_passphrases_2 = passphrases
        .iter()
        .filter(|s| are_valid_passphrases_annagram_free(s))
        .count();
    println!("Solution 1: {}\n", valid_passphrases_1);
    println!("Solution 2: {}\n", valid_passphrases_2);

    println!("*** Day 5: A Maze of Twisty Trampolines, All Alike ***");
    println!("Input: {}", DAY_5_INPUT);
    println!("Solution 1: {}\n", steps_to_escape(DAY_5_INPUT)?);
    println!("Solution 2: {}\n", steps_to_escape_next(DAY_5_INPUT)?);

    println!("*** Day 6: Memory Reallocation ***");
    println!("Input: {}", DAY_6_INPUT);
    let mut redistributer = RedistributionCycles::new(DAY_6_INPUT);
    println!("Solution 1: {:?}\n", redistributer.redist()?);
    println!("Solution 2: {:?}\n", redistributer.loop_size()?);

    println!("*** Day 7: Recursive Circus ***");
    println!("Input: {}", DAY_7_INPUT);
    let tree = Node::from_str(DAY_7_INPUT)?;
    println!("Solution 1: {:?}\n", tree.name);
    let with_kid_weights = NodeWithChildrenWeight::build(&tree);
    println!(
        "Solution 2: {:?}\n",
        with_kid_weights.smallest_rebalanced_children_weight()
    );

    println!("*** Day 8: I Heard You Like Registers ***");
    println!("Input: {}", DAY_8_INPUT);
    println!("Solutions: {:?}\n", simualate_instructions(DAY_8_INPUT));

    println!("*** Day 9: Stream Processing ***");
    println!("Input: {}", DAY_9_INPUT);
    println!("Solution: {}\n", count_groups(DAY_9_INPUT));

    println!("*** Day 10: Knot Hash ***");
    println!("Input: {}", DAY_10_INPUT);
    println!("Solution1: {}\n", solve_knot_hash(DAY_10_INPUT)?);
    println!("Solution2: {}\n", hex_knot_hash(DAY_10_INPUT)?);

    println!("*** Day 11: Hex Ed ***");
    println!("Input: {}", DAY_11_INPUT);
    println!("Solution1: {:?}\n", hex_steps_from_centre(DAY_11_INPUT));

    Ok(())
}
