use std::collections::HashMap;
use std::hash::Hash;
use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

const DAY_8_INPUT: &str = include_str!("../data/day_8_input");

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 8: I Heard You Like Registers ***");
    println!("Input: {}", DAY_8_INPUT);
    println!("Solutions: {:?}\n", simualate_instructions(DAY_8_INPUT));
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    register: String,
    op: Op,
    amount: i64,
    check_register: String,
    cond: Cond,
    cond_amount: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
enum Cond {
    GT,
    LT,
    GTE,
    LTE,
    E,
    NE,
}

impl Cond {
    fn compare(&self, target: i64, amount: i64) -> bool {
        match self {
            &Cond::GT => target > amount,
            &Cond::LT => target < amount,
            &Cond::GTE => target >= amount,
            &Cond::LTE => target <= amount,
            &Cond::E => target == amount,
            &Cond::NE => target != amount,
        }
    }
}

struct Simulation<'a> {
    instructions: &'a Vec<Instruction>,
    registers: HashMap<&'a str, i64>,
    historical_highest_reg_value: Option<i64>,
    current_highest_reg_value: Option<i64>,
}

impl<'a> Simulation<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Simulation<'a> {
        Simulation {
            instructions: instructions,
            registers: HashMap::new(),
            historical_highest_reg_value: None,
            current_highest_reg_value: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SimulationResult {
    historical_highest_reg_value: Option<i64>,
    current_highest_reg_value: Option<i64>,
}

fn run_simulation(s: &mut Simulation) -> () {
    s.instructions.iter().fold(s, |simulation, i| {
        // arg borrow checker..
        let should_proceed = {
            let check_register_value = simulation
                .registers
                .get(i.check_register.as_str())
                .unwrap_or(&0);
            i.cond.compare(*check_register_value, i.cond_amount)
        };
        if should_proceed {
            match i.op {
                Op::Inc => {
                    *simulation.registers.entry(i.register.as_str()).or_insert(0) += i.amount
                }
                Op::Dec => {
                    *simulation.registers.entry(i.register.as_str()).or_insert(0) -= i.amount
                }
            }
        }
        let max_value_in_reg = max_value(&simulation.registers);
        simulation.current_highest_reg_value = max_value_in_reg;
        match max_value_in_reg {
            None => (),
            Some(v) => match simulation.historical_highest_reg_value {
                Some(old_highest_reg_value) => if v > old_highest_reg_value {
                    simulation.historical_highest_reg_value = Some(v)
                },
                None => simulation.historical_highest_reg_value = Some(v),
            },
        }
        simulation
    });
}

fn simualate_instructions(s: &str) -> Result<SimulationResult, Errors<PointerOffset, char, &str>> {
    let (instructions, _) = Instruction::parse(s)?;
    let mut simulation = Simulation::new(&instructions);
    run_simulation(&mut simulation);
    Ok(SimulationResult {
        current_highest_reg_value: simulation.current_highest_reg_value,
        historical_highest_reg_value: simulation.historical_highest_reg_value,
    })
}

fn max_value<'a, K, V>(hash: &'a HashMap<K, V>) -> Option<V>
where
    K: Eq + Hash,
    V: Copy + Ord,
{
    hash.iter().fold(None, |acc, (_, v)| match acc {
        None => Some(*v),
        Some(old_v) => if *v > old_v {
            Some(*v)
        } else {
            acc
        },
    })
}

macro_rules! instruction_parser {
    () => {
        {
        let identifier_parser = many1::<String, _>(letter());
        let op_parser = try(string("inc"))
            .map(|_| Op::Inc)
            .or(string("dec").map(|_| Op::Dec));
        let amount_parser = number_parser!(i64);
        let cond_statement_parser = string("if");
        let cond_target_parser = many1::<String, _>(letter());
        let cond_parser = (try(string(">=")).map(|_| Cond::GTE))
            .or(try(string("<=")).map(|_| Cond::LTE))
            .or(try(string("==")).map(|_| Cond::E))
            .or(try(string("!=")).map(|_| Cond::NE))
            .or(try(string(">")).map(|_| Cond::GT))
            .or(string("<").map(|_| Cond::LT));
        let cond_amount_parser = number_parser!(i64);
        identifier_parser
            .skip(tabs_or_spaces!())
            .and(op_parser.skip(tabs_or_spaces!()))
            .and(amount_parser.skip(tabs_or_spaces!()))
            .skip(cond_statement_parser.skip(tabs_or_spaces!()))
            .and(cond_target_parser.skip(tabs_or_spaces!()))
            .and(cond_parser.skip(tabs_or_spaces!()))
            .and(cond_amount_parser)
            .map(
                |(
                    (
                        (((parsed_identifier, parsed_op), parsed_amount), parsed_cond_target),
                        parsed_cond,
                    ),
                    parsed_cond_amount,
                )| {
                    Instruction {
                        register: parsed_identifier,
                        op: parsed_op,
                        amount: parsed_amount,
                        check_register: parsed_cond_target,
                        cond: parsed_cond,
                        cond_amount: parsed_cond_amount,
                    }
                },
            )
        }
    }
}

impl Instruction {
    // Wish I could separate this out into functions, but the whole impl trait story in Rust
    // makes that __reaally__ difficult
    fn parse(s: &str) -> Result<(Vec<Instruction>, &str), Errors<PointerOffset, char, &str>> {
        let mut instructions_parser =
            skip_many(newline()).with(sep_by(instruction_parser!(), spaces()));
        instructions_parser.easy_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use day_8::*;

    const TEST_INPUT: &str = r#"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#;

    #[test]
    fn parse_test() {
        let (parsed, _) = Instruction::parse(TEST_INPUT).unwrap();
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn parse_real_test() {
        let (parsed, _) = Instruction::parse(DAY_8_INPUT).unwrap();
        assert!(parsed.len() > 1);
    }

    #[test]
    fn simualate_instructions_test() {
        let results = simualate_instructions(TEST_INPUT);
        assert_eq!(
            results,
            Ok(SimulationResult {
                historical_highest_reg_value: Some(10),
                current_highest_reg_value: Some(1),
            })
        );
    }

    #[test]
    fn simualate_instructions_real_test() {
        let results = simualate_instructions(DAY_8_INPUT);
        assert_eq!(
            results,
            Ok(SimulationResult {
                historical_highest_reg_value: Some(7037),
                current_highest_reg_value: Some(4902),
            })
        )
    }

}
