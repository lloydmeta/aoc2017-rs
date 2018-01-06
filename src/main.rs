extern crate aoc_2017;
extern crate clap;

use std::str::FromStr;
use std::fmt::Display;

use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::process::exit;

use aoc_2017::day_1;
use aoc_2017::day_2;
use aoc_2017::day_3;
use aoc_2017::day_4;
use aoc_2017::day_5;
use aoc_2017::day_6;
use aoc_2017::day_7;
use aoc_2017::day_8;
use aoc_2017::day_9;
use aoc_2017::day_10;
use aoc_2017::day_11;
use aoc_2017::day_12;
use aoc_2017::day_13;
use aoc_2017::day_14;
use aoc_2017::day_15;
use aoc_2017::day_16;
use aoc_2017::day_17;
use aoc_2017::day_18;

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
    let matches = App::new("Advent of Code 2017")
        .version(version().as_str())
        .about("Solutions to AoC 2017 !")
        .arg(
            Arg::with_name("day")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Which day's solution you want to run"),
        )
        .get_matches();
    match get_number("day", Some(0), &matches) {
        1 => day_1::run()?,
        2 => day_2::run()?,
        3 => day_3::run()?,
        4 => day_4::run()?,
        5 => day_5::run()?,
        6 => day_6::run()?,
        7 => day_7::run()?,
        8 => day_8::run()?,
        9 => day_9::run()?,
        10 => day_10::run()?,
        11 => day_11::run()?,
        12 => day_12::run()?,
        13 => day_13::run()?,
        14 => day_14::run()?,
        15 => day_15::run()?,
        16 => day_16::run()?,
        17 => day_17::run()?,
        18 => day_18::run()?,
        other => Err(format!("Invalid day: {}", other))?,
    }
    Ok(())
}

fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn get_number<'a, A>(name: &str, maybe_min: Option<A>, matches: &ArgMatches<'a>) -> A
where
    A: FromStr + PartialOrd + Display + Copy,
    <A as FromStr>::Err: std::fmt::Debug,
{
    matches
        .value_of(name)
        .and_then(|s| s.parse::<A>().ok())
        .and_then(|u| match maybe_min {
            Some(min) => if u > min {
                Some(u)
            } else {
                None
            },
            _ => Some(u),
        })
        .expect(
            &{
                if let Some(min) = maybe_min {
                    format!("{} should be a positive number greater than {}.", name, min)
                } else {
                    format!("{} should be a positive number.", name)
                }
            }[..],
        )
}
