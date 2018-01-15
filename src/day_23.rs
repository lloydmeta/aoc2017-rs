use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

use self::Op::*;

use std::collections::HashMap;
use std::error::Error;

const DAY_23_INPUT: &'static str = include_str!("../data/day_23_input");

pub fn run() -> Result<(), Box<Error>> {
    println!("*** Day 23: Coprocessor Conflagration ***");
    let ops = Op::parse_many(DAY_23_INPUT)?;
    println!("Input: {}", DAY_23_INPUT);
    println!("Solution 1: {}", solution_1(&ops));
    println!("Solution 2: {:?}", solution_2(&ops));
    Ok(())
}

fn solution_1(ops: &Vec<Op>) -> usize {
    let mut machine = StatefulMachine::new();
    machine.run(ops);
    machine.mul_count
}

// Reddit told me this is just a composite-numbers count
fn solution_2(ops: &Vec<Op>) -> usize {
    let init_b = ops.iter()
        .filter_map(|op| match op {
            &SetRegNum { reg, num } if reg == 'b' => Some(num),
            _ => None,
        })
        .next()
        .unwrap_or(0);
    let start = init_b * 100 + 100000;
    let end = start + 17000 + 17;
    // http://planetmath.org/howtofindwhetheragivennumberisprimeornot
    fn is_composite(n: isize) -> bool {
        let sqrt = (n as f64).sqrt() as isize;
        (2..sqrt).any(|v| n % v == 0)
    }
    let mut composites_count = 0;
    let mut current = start;
    while current <= end {
        if is_composite(current) {
            composites_count += 1;
        }
        current += 17;
    }
    composites_count
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    SetRegNum { reg: char, num: isize },
    SetRegReg { reg_target: char, reg_source: char },
    SubRegNum { reg: char, num: isize },
    SubRegReg { reg_target: char, reg_other: char },
    MulRegNum { reg: char, num: isize },
    MulRegReg { reg_target: char, reg_other: char },
    JnzRegNum { reg_check: char, num_amount: isize },
    JnzRegReg { reg_check: char, reg_amount: char },
    JnzNumNum { num_check: isize, num_amount: isize },
    JnzNumReg { num_check: isize, reg_amount: char },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Event {
    op: Op,
    current_idx: isize,
}

struct StatefulMachine {
    current_idx: isize,
    registers: HashMap<char, isize>,
    mul_count: usize,
}

const DEFAULT: isize = 0;

impl StatefulMachine {
    fn new() -> StatefulMachine {
        StatefulMachine {
            current_idx: 0,
            registers: HashMap::new(),
            mul_count: 0,
        }
    }

    fn run(&mut self, ops: &Vec<Op>) -> () {
        let ops_len = ops.len() as isize;
        while self.current_idx >= 0 && self.current_idx < ops_len {
            if let Some(op) = ops.get(self.current_idx as usize) {
                self.interpret(&op);
                match op {
                    &MulRegNum { .. } | &MulRegReg { .. } => {
                        self.mul_count += 1;
                        self.current_idx += 1;
                    }
                    // Skip incr. for jump instructions
                    &JnzNumNum { .. }
                    | &JnzNumReg { .. }
                    | &JnzRegNum { .. }
                    | &JnzRegReg { .. } => (),
                    _ => self.current_idx += 1,
                }
            }
        }
    }

    fn interpret(&mut self, op: &Op) -> () {
        match op {
            &SetRegNum { reg, num } => {
                self.registers.insert(reg, num);
            }
            &SetRegReg {
                reg_target,
                reg_source,
            } => {
                let num = self.get_register_value(reg_source);
                self.registers.insert(reg_target, num);
            }
            &SubRegNum { reg, num } => {
                *self.registers.entry(reg).or_insert(DEFAULT) -= num;
            }
            &SubRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                *self.registers.entry(reg_target).or_insert(DEFAULT) -= num;
            }
            &MulRegNum { reg, num } => {
                *self.registers.entry(reg).or_insert(DEFAULT) *= num;
            }
            &MulRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                *self.registers.entry(reg_target).or_insert(DEFAULT) *= num;
            }
            &JnzNumNum {
                num_check,
                num_amount,
            } => if num_check != 0 {
                self.current_idx += num_amount;
            } else {
                self.current_idx += 1;
            },
            &JnzNumReg {
                num_check,
                reg_amount,
            } => if num_check != 0 {
                self.current_idx += self.get_register_value(reg_amount);
            } else {
                self.current_idx += 1;
            },
            &JnzRegNum {
                reg_check,
                num_amount,
            } => if self.get_register_value(reg_check) != 0 {
                self.current_idx += num_amount;
            } else {
                self.current_idx += 1;
            },
            &JnzRegReg {
                reg_check,
                reg_amount,
            } => if self.get_register_value(reg_check) != 0 {
                self.current_idx += self.get_register_value(reg_amount);
            } else {
                self.current_idx += 1;
            },
        }
    }

    fn get_register_value(&self, reg: char) -> isize {
        self.registers.get(&reg).map(|x| *x).unwrap_or(DEFAULT)
    }

    #[allow(dead_code)]
    fn print_debug(&self) -> () {
        println!(" ~~ State info ~~");
        println!("current_idx: {}", self.current_idx);
        println!("Registers:");
        for (reg, val) in self.registers.iter() {
            println!("{}: {}", reg, val);
        }
    }
}

macro_rules! set_parser {
    () => {
        string("set")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg, num)| {
                                SetRegNum { reg, num }
                             })
                    )
                    .or(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(reg_target, reg_source)| {
                                SetRegReg { reg_target, reg_source }
                             })
                    )
                )
            )
    };
}

macro_rules! sub_parser {
    () => {
        string("sub")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg, num)| {
                                SubRegNum { reg, num }
                             })
                    )
                    .or(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(reg_target, reg_other)| {
                                SubRegReg { reg_target, reg_other }
                             })
                    )
                )
            )
    };
}

macro_rules! mul_parser {
    () => {
        string("mul")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg, num)| {
                                MulRegNum { reg, num }
                             })
                    )
                    .or(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(reg_target, reg_other)| {
                                MulRegReg { reg_target, reg_other }
                             })
                    )
                )
            )
    };
}

macro_rules! jnz_parser {
    () => {
        string("jnz")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg_check, num_amount)| {
                                JnzRegNum { reg_check, num_amount }
                             })
                    )
                    .or(
                        try(
                            letter()
                            .and(
                                tabs_or_spaces!()
                                .with(letter())
                            ).map(|(reg_check, reg_amount)| {
                                    JnzRegReg { reg_check, reg_amount }
                                })
                        )
                    )
                    .or(
                        try(
                            number_parser!(isize)
                            .and(
                                tabs_or_spaces!()
                                .with(number_parser!(isize))
                            ).map(|(num_check, num_amount)| {
                                    JnzNumNum { num_check, num_amount }
                                })
                        )
                    )
                    .or(
                        number_parser!(isize)
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(num_check, reg_amount)| {
                                JnzNumReg { num_check, reg_amount }
                            })
                    )
                )
            )
    };
}

macro_rules! op_parser {
    () => {
        try(
            set_parser!()
        )
        .or(
            try(
                sub_parser!()
            )
        ).or(
            try(
                mul_parser!()
            )
        ).or(
            jnz_parser!()
        )
    };
}

impl Op {
    fn parse_many(s: &str) -> Result<Vec<Op>, Errors<PointerOffset, char, &str>> {
        let mut parser = spaces().with(sep_by(op_parser!(), spaces()));
        let (ops, _) = parser.easy_parse(s)?;
        Ok(ops)
    }
}

#[cfg(test)]
mod tests {
    use day_23::*;

    #[test]
    fn op_parse_many_real_test() {
        let ops = Op::parse_many(DAY_23_INPUT).unwrap();
        assert_eq!(ops.len(), 32);
    }

    #[test]
    fn solution_1_test() {
        let ops = Op::parse_many(DAY_23_INPUT).unwrap();
        assert_eq!(solution_1(&ops), 8281);
    }
    #[test]
    fn solution_2_test() {
        let ops = Op::parse_many(DAY_23_INPUT).unwrap();
        assert_eq!(solution_2(&ops), 911);
    }

}
