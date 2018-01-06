use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

use self::Op::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::{BorrowError, BorrowMutError};

pub fn run() -> Result<(), Box<Error>> {
    println!("*** Day 18: Duet ***");
    let ops = Op::parse_many(DAY_18_INPUT)?.0;
    println!("solution 1: {:?}", solution_1(&ops));
    println!("solution 2: {:?}", solution_2(&ops)?);
    Ok(())
}

fn solution_1(ops: &Vec<Op>) -> isize {
    let mut state = StatefulMachine::new(|op, state| match op {
        &RcvReg { reg } => if let Some(num) = state.registers.get(&reg) {
            *num != 0
        } else {
            false
        },
        _ => false,
    });
    state.run(&ops);
    state.freqs.last
}

fn solution_2(ops: &Vec<Op>) -> Result<Option<usize>, Box<Error>> {
    let mut rts = RTS::new();
    let _ = rts.run(&ops)?;
    let fibre_1 = rts.fibres.iter().find(|f| f.id == 1);
    let fibre_1_snd_count = fibre_1.map(|f| {
        let log = &f.log;
        log.iter().filter(|ev| {
            match ev.op {
                SndReg { ..} | SndNum {..} => true,
                _ => false
            }
        }).count()
    });
    Ok(fibre_1_snd_count)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    SndReg { reg: char },
    SndNum { num: isize },
    SetRegNum { reg: char, num: isize },
    SetRegReg { reg_target: char, reg_source: char },
    AddRegNum { reg: char, num: isize },
    AddRegReg { reg_target: char, reg_other: char },
    MulRegNum { reg: char, num: isize },
    MulRegReg { reg_target: char, reg_other: char },
    ModRegNum { reg: char, num: isize },
    ModRegReg { reg_target: char, reg_other: char },
    RcvReg { reg: char },
    JgzRegNum { reg_check: char, num_amount: isize },
    JgzRegReg { reg_check: char, reg_amount: char },
    JgzNumNum { num_check: isize, num_amount: isize },
    JgzNumReg { num_check: isize, reg_amount: char },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Freqs {
    current: isize,
    last: isize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Event {
    op: Op,
    current_idx: isize,
    before: Freqs,
    after: Freqs,
}

struct StatefulMachine<F>
where
    F: Fn(&Op, &StatefulMachine<F>) -> bool,
{
    current_idx: isize,
    freqs: Freqs,
    registers: HashMap<char, isize>,
    break_when: F,
    log: Vec<Event>,
}

const DEFAULT: isize = 0;

impl<F> StatefulMachine<F>
where
    F: Fn(&Op, &StatefulMachine<F>) -> bool,
{
    fn new(break_when: F) -> StatefulMachine<F> {
        StatefulMachine {
            current_idx: 0,
            freqs: Freqs {
                current: 0,
                last: 0,
            },
            registers: HashMap::new(),
            log: Vec::new(),
            break_when: break_when,
        }
    }

    fn run(&mut self, ops: &Vec<Op>) -> () {
        let ops_len = ops.len() as isize;
        while self.current_idx >= 0 && self.current_idx < ops_len {
            if let Some(op) = ops.get(self.current_idx as usize) {
                let before_freqs = self.freqs;
                self.interpret(&op);
                let after_freqs = self.freqs;
                self.log.push(Event {
                    op: *op,
                    current_idx: self.current_idx,
                    before: before_freqs,
                    after: after_freqs,
                });
                if (self.break_when)(op, self) {
                    break;
                }
                match op {
                    // Skip incr. for jump instructions
                    &JgzNumNum { .. }
                    | &JgzNumReg { .. }
                    | &JgzRegNum { .. }
                    | &JgzRegReg { .. } => (),
                    _ => self.current_idx += 1,
                }
            }
        }
    }

    fn interpret(&mut self, op: &Op) -> () {
        use day_18::Op::*;
        match op {
            &SndNum { num } => {
                self.freqs.last = self.freqs.current;
                self.freqs.current = num;
            }
            &SndReg { reg } => {
                let num = self.get_register_value(reg);
                self.freqs.last = self.freqs.current;
                self.freqs.current = num;
            }
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
            &AddRegNum { reg, num } => {
                *self.registers.entry(reg).or_insert(DEFAULT) += num;
            }
            &AddRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                *self.registers.entry(reg_target).or_insert(DEFAULT) += num;
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
            &ModRegNum { reg, num } => if num != 0 {
                *self.registers.entry(reg).or_insert(DEFAULT) %= num;
            },
            &ModRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                if num != 0 {
                    *self.registers.entry(reg_target).or_insert(DEFAULT) %= num;
                }
            }
            &RcvReg { reg } => {
                let num = self.get_register_value(reg);
                if num != 0 {
                    let last = self.freqs.last;
                    self.freqs.last = self.freqs.current;
                    self.freqs.current = last;
                }
            }
            &JgzNumNum {
                num_check,
                num_amount,
            } => if num_check > 0 {
                self.current_idx += num_amount;
            } else {
                self.current_idx += 1;
            },
            &JgzNumReg {
                num_check,
                reg_amount,
            } => if num_check > 0 {
                self.current_idx += self.get_register_value(reg_amount);
            } else {
                self.current_idx += 1;
            },
            &JgzRegNum {
                reg_check,
                num_amount,
            } => if self.get_register_value(reg_check) > 0 {
                self.current_idx += num_amount;
            } else {
                self.current_idx += 1;
            },
            &JgzRegReg {
                reg_check,
                reg_amount,
            } => if self.get_register_value(reg_check) > 0 {
                self.current_idx += self.get_register_value(reg_amount);
            } else {
                self.current_idx += 1;
            },
        }
    }

    fn get_register_value(&self, reg: char) -> isize {
        self.registers.get(&reg).map(|x| *x).unwrap_or(DEFAULT)
    }

    #[test]
    fn print_debug(&self) -> () {
        println!(" ~~ State info ~~");
        println!("current_idx: {}", self.current_idx);
        println!("{:?}", self.freqs);
        println!("Log:");
        for ev in self.log.iter() {
            println!("{:?}", ev);
        }
        println!("Registers:");
        for (reg, val) in self.registers.iter() {
            println!("{}: {}", reg, val);
        }
    }
}

struct RTS {
    fibres: Vec<Fibre>,
}

impl RTS {
    fn new() -> RTS {
        // hard-coded to just 2 Fibres that send things to each other
        let mailbox_0 = Rc::new(RefCell::new(VecDeque::new()));
        let mailbox_1 = Rc::new(RefCell::new(VecDeque::new()));

        let fibre_0 = Fibre::new(0, mailbox_0.clone(), mailbox_1.clone());
        let fibre_1 = Fibre::new(1, mailbox_1.clone(), mailbox_0.clone());
        RTS {
            fibres: vec![fibre_0, fibre_1],
        }
    }

    fn run(&mut self, ops: &Vec<Op>) -> Result<(), Box<Error>> {
        while let Some(runnable_fibre) = self.fibres
            .iter_mut()
            .find(|i| i.can_run(ops).unwrap_or(false))
        {
            runnable_fibre.run(ops)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct FibreEvent {
    op: Op,
    current_idx: isize,
}

struct Fibre {
    id: usize,
    current_idx: isize,
    rcv_wait: bool,
    registers: HashMap<char, isize>,
    log: Vec<FibreEvent>,
    rcv: Rc<RefCell<VecDeque<isize>>>,
    snd: Rc<RefCell<VecDeque<isize>>>,
}

impl Fibre {
    fn new(
        id: usize,
        rcv: Rc<RefCell<VecDeque<isize>>>,
        snd: Rc<RefCell<VecDeque<isize>>>,
    ) -> Fibre {
        let registers = {
            let mut t = HashMap::new();
            t.insert('p', id as isize);
            t
        };
        Fibre {
            id: id,
            current_idx: 0,
            rcv_wait: false,
            registers: registers,
            log: Vec::new(),
            rcv: rcv,
            snd: snd,
        }
    }

    fn run(&mut self, ops: &Vec<Op>) -> Result<(), Box<Error>> {
        while self.can_run(ops)? {
            if let Some(op) = ops.get(self.current_idx as usize) {
                let _ = self.interpret(&op)?;
                self.log.push(FibreEvent {
                    op: *op,
                    current_idx: self.current_idx,
                });
                match op {
                    // Skip incr. for jump instructions and rcv instructions
                    &JgzNumNum { .. }
                    | &JgzNumReg { .. }
                    | &JgzRegNum { .. }
                    | &JgzRegReg { .. }
                    | &RcvReg { .. } => (),
                    _ => self.current_idx += 1,
                }
            }
        }
        Ok(())
    }

    fn can_run(&self, ops: &Vec<Op>) -> Result<bool, BorrowError> {
        let ops_len = ops.len() as isize;
        Ok(self.current_idx >= 0 && self.current_idx < ops_len && !self.is_blocked()?)
    }

    fn is_blocked(&self) -> Result<bool, BorrowError> {
        Ok(self.rcv_wait && self.rcv.try_borrow()?.len() == 0)
    }

    fn interpret(&mut self, op: &Op) -> Result<(), BorrowMutError> {
        use day_18::Op::*;
        match op {
            &SndNum { num } => Ok(self.snd.try_borrow_mut()?.push_back(num)),
            &SndReg { reg } => {
                let num = self.get_register_value(reg);
                Ok(self.snd.try_borrow_mut()?.push_back(num))
            }
            &SetRegNum { reg, num } => {
                self.registers.insert(reg, num);
                Ok(())
            }
            &SetRegReg {
                reg_target,
                reg_source,
            } => {
                let num = self.get_register_value(reg_source);
                self.registers.insert(reg_target, num);
                Ok(())
            }
            &AddRegNum { reg, num } => Ok(*self.registers.entry(reg).or_insert(DEFAULT) += num),
            &AddRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                Ok(*self.registers.entry(reg_target).or_insert(DEFAULT) += num)
            }
            &MulRegNum { reg, num } => Ok(*self.registers.entry(reg).or_insert(DEFAULT) *= num),
            &MulRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                Ok(*self.registers.entry(reg_target).or_insert(DEFAULT) *= num)
            }
            &ModRegNum { reg, num } => {
                if num != 0 {
                    *self.registers.entry(reg).or_insert(DEFAULT) %= num
                }
                Ok(())
            }
            &ModRegReg {
                reg_target,
                reg_other,
            } => {
                let num = self.get_register_value(reg_other);
                if num != 0 {
                    *self.registers.entry(reg_target).or_insert(DEFAULT) %= num;
                }
                Ok(())
            }
            &RcvReg { reg } => if let Some(v) = self.rcv.try_borrow_mut()?.pop_front() {
                self.registers.insert(reg, v);
                self.rcv_wait = false;
                self.current_idx += 1;
                Ok(())
            } else {
                // set rcv_wait to true, but don't bump the current_idx
                // so we can resume back there once we get a message
                self.rcv_wait = true;
                Ok(())
            },
            &JgzNumNum {
                num_check,
                num_amount,
            } => {
                if num_check > 0 {
                    self.current_idx += num_amount;
                } else {
                    self.current_idx += 1;
                }
                Ok(())
            }
            &JgzNumReg {
                num_check,
                reg_amount,
            } => {
                if num_check > 0 {
                    self.current_idx += self.get_register_value(reg_amount);
                } else {
                    self.current_idx += 1;
                }
                Ok(())
            }
            &JgzRegNum {
                reg_check,
                num_amount,
            } => {
                if self.get_register_value(reg_check) > 0 {
                    self.current_idx += num_amount;
                } else {
                    self.current_idx += 1;
                }
                Ok(())
            }
            &JgzRegReg {
                reg_check,
                reg_amount,
            } => {
                if self.get_register_value(reg_check) > 0 {
                    self.current_idx += self.get_register_value(reg_amount);
                } else {
                    self.current_idx += 1;
                }
                Ok(())
            }
        }
    }

    fn get_register_value(&self, reg: char) -> isize {
        self.registers.get(&reg).map(|x| *x).unwrap_or(DEFAULT)
    }

    #[test]
    fn print_debug(&self) -> () {
        println!(" ~~ State info ~~");
        println!("current_idx: {}", self.current_idx);
        println!("Log:");
        for ev in self.log.iter() {
            println!("{:?}", ev);
        }
        println!("Registers:");
        for (reg, val) in self.registers.iter() {
            println!("{}: {}", reg, val);
        }
    }
}

macro_rules! snd_parser {
    () => {
        string("snd")
            .with(
                tabs_or_spaces!()
                .with(
                    try(letter().map(|reg| SndReg { reg }))
                    .or(number_parser!(isize).map(|num| SndNum { num } ))
                )
            )
    };
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

macro_rules! add_parser {
    () => {
        string("add")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg, num)| {
                                AddRegNum { reg, num }
                             })
                    )
                    .or(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(reg_target, reg_other)| {
                                AddRegReg { reg_target, reg_other }
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

macro_rules! mod_parser {
    () => {
        string("mod")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg, num)| {
                                ModRegNum { reg, num }
                             })
                    )
                    .or(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(reg_target, reg_other)| {
                                ModRegReg { reg_target, reg_other }
                             })
                    )
                )
            )
    };
}

macro_rules! rcv_parser {
    () => {
        string("rcv")
            .with(
                tabs_or_spaces!()
                .with(
                    letter().map(|reg| RcvReg { reg })
                )
            )
    };
}

macro_rules! jgz_parser {
    () => {
        string("jgz")
            .with(
                tabs_or_spaces!()
                .with(
                    try(
                        letter()
                        .and(
                            tabs_or_spaces!()
                            .with(number_parser!(isize))
                        ).map(|(reg_check, num_amount)| {
                                JgzRegNum { reg_check, num_amount }
                             })
                    )
                    .or(
                        try(
                            letter()
                            .and(
                                tabs_or_spaces!()
                                .with(letter())
                            ).map(|(reg_check, reg_amount)| {
                                    JgzRegReg { reg_check, reg_amount }
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
                                    JgzNumNum { num_check, num_amount }
                                })
                        )
                    )
                    .or(
                        number_parser!(isize)
                        .and(
                            tabs_or_spaces!()
                            .with(letter())
                        ).map(|(num_check, reg_amount)| {
                                JgzNumReg { num_check, reg_amount }
                            })
                    )
                )
            )
    };
}

macro_rules! op_parser {
    () => {
        try(
            snd_parser!()
        ).or(
            try(
                set_parser!()
            )
        ).or(
            try(
                add_parser!()
            )
        ).or(
            try(
                mul_parser!()
            )
        ).or(
            try(
                mod_parser!()
            )
        ).or(
            try(
                rcv_parser!()
            )
        ).or(
            jgz_parser!()
        )
    };
}

impl Op {
    fn parse_many(s: &str) -> Result<(Vec<Op>, &str), Errors<PointerOffset, char, &str>> {
        let mut parser = spaces().with(sep_by(op_parser!(), spaces()));
        parser.easy_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use day_18::*;

    const TEST_INPUT: &'static str = r#"
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#;

    #[test]
    fn snd_parser_test() {
        fn parse(s: &str) -> Op {
            snd_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("snd a");
        assert_eq!(parsed_op_1, SndReg { reg: 'a' });
        let parsed_op_2 = parse("snd 10");
        assert_eq!(parsed_op_2, SndNum { num: 10 });
        let parsed_op_3 = parse("snd -10");
        assert_eq!(parsed_op_3, SndNum { num: -10 });
    }

    #[test]
    fn set_parser_test() {
        fn parse(s: &str) -> Op {
            set_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("set a 10");
        assert_eq!(parsed_op_1, SetRegNum { reg: 'a', num: 10 });
        let parsed_op_2 = parse("set a -10");
        assert_eq!(parsed_op_2, SetRegNum { reg: 'a', num: -10 });
        let parsed_op_3 = parse("set a b");
        assert_eq!(
            parsed_op_3,
            SetRegReg {
                reg_target: 'a',
                reg_source: 'b',
            }
        );
    }

    #[test]
    fn add_parser_test() {
        fn parse(s: &str) -> Op {
            add_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("add a 10");
        assert_eq!(parsed_op_1, AddRegNum { reg: 'a', num: 10 });
        let parsed_op_2 = parse("add a -10");
        assert_eq!(parsed_op_2, AddRegNum { reg: 'a', num: -10 });
        let parsed_op_3 = parse("add a b");
        assert_eq!(
            parsed_op_3,
            AddRegReg {
                reg_target: 'a',
                reg_other: 'b',
            }
        );
    }

    #[test]
    fn mul_parser_test() {
        fn parse(s: &str) -> Op {
            mul_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("mul a 10");
        assert_eq!(parsed_op_1, MulRegNum { reg: 'a', num: 10 });
        let parsed_op_2 = parse("mul a -10");
        assert_eq!(parsed_op_2, MulRegNum { reg: 'a', num: -10 });
        let parsed_op_3 = parse("mul a b");
        assert_eq!(
            parsed_op_3,
            MulRegReg {
                reg_target: 'a',
                reg_other: 'b',
            }
        );
    }

    #[test]
    fn mod_parser_test() {
        fn parse(s: &str) -> Op {
            mod_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("mod a 10");
        assert_eq!(parsed_op_1, ModRegNum { reg: 'a', num: 10 });
        let parsed_op_2 = parse("mod a -10");
        assert_eq!(parsed_op_2, ModRegNum { reg: 'a', num: -10 });
        let parsed_op_3 = parse("mod a b");
        assert_eq!(
            parsed_op_3,
            ModRegReg {
                reg_target: 'a',
                reg_other: 'b',
            }
        );
    }

    #[test]
    fn rcv_parser_test() {
        fn parse(s: &str) -> Op {
            rcv_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("rcv a");
        assert_eq!(parsed_op_1, RcvReg { reg: 'a' });
    }

    #[test]
    fn jgz_parser_test() {
        fn parse(s: &str) -> Op {
            jgz_parser!().easy_parse(s).unwrap().0
        }
        let parsed_op_1 = parse("jgz a 10");
        assert_eq!(
            parsed_op_1,
            JgzRegNum {
                reg_check: 'a',
                num_amount: 10,
            }
        );
        let parsed_op_2 = parse("jgz a b");
        assert_eq!(
            parsed_op_2,
            JgzRegReg {
                reg_check: 'a',
                reg_amount: 'b',
            }
        );
        let parsed_op_3 = parse("jgz 10 -10");
        assert_eq!(
            parsed_op_3,
            JgzNumNum {
                num_check: 10,
                num_amount: -10,
            }
        );
        let parsed_op_4 = parse("jgz -10 a");
        assert_eq!(
            parsed_op_4,
            JgzNumReg {
                num_check: -10,
                reg_amount: 'a',
            }
        );
    }

    #[test]
    fn op_parse_many_test() {
        let r = Op::parse_many(TEST_INPUT).unwrap().0;
        assert_eq!(r.len(), 10);
    }

    #[test]
    fn op_parse_many_real_test() {
        let r = Op::parse_many(DAY_18_INPUT).unwrap().0;
        assert_eq!(r.len(), 41);
    }

    #[test]
    fn first_half_real_test() {
        let ops = Op::parse_many(DAY_18_INPUT).unwrap().0;
        assert_eq!(solution_1(&ops), 4601);
    }

    #[test]
    fn second_half_real_test() {
        let ops = Op::parse_many(DAY_18_INPUT).unwrap().0;
        assert_eq!(solution_2(&ops).unwrap(), Some(6858));
    }

}

const DAY_18_INPUT: &'static str = r#"
set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 952
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19"#;
