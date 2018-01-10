use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

use std::collections::HashMap;
use std::error::Error;

const DAY_16_INPUT: &'static str = include_str!("../data/day_16_input");

pub fn run() -> Result<(), Box<Error>> {
    println!("*** Day 16: Permutation Promenade ***");
    println!("Input {:?}", DAY_16_INPUT);
    let mut line = input_line();
    println!("Init {:?}", line);
    let (ops, _) = Op::parse_many(DAY_16_INPUT)?;
    let mut dancer = Dancer::new(&ops, &mut line);
    dancer.dance(1);
    let solution_1: String = dancer.line.iter().collect();
    println!("Solution1: {}", solution_1);
    println!("Come back later...");
    dancer.dance(1000000000 - 1);
    let solution_2: String = dancer.line.iter().collect();
    println!("Solution2: {}", solution_2);
    Ok(())
}

fn input_line() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
    ]
}

macro_rules! spin_parser {
    () => {
        char('s').with(many1::<String, _>(digit()).and_then(|s| {
            s.parse::<usize>()
        })).map(|spin_last| Op::Spin(spin_last))
    }
}

macro_rules! exchange_parser {
    () => {
        char('x').with(
            many1::<String, _>(digit())
                .and_then(|s| s.parse::<usize>())
            .skip(char('/'))
            .and(
             many1::<String, _>(digit())
                .and_then(|s| s.parse::<usize>())
            )
        ).map(|(a, b)| Op::Exchange(a, b))
    }
}

macro_rules! partner_parser {
    () => {
        char('p').with(
            letter()
            .skip(char('/'))
            .and(letter())
        ).map(|(a, b)| Op::Partner(a, b))
    }
}

macro_rules! op_parser {
     () => {
        spin_parser!()
            .or(exchange_parser!())
            .or(partner_parser!())
     }
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Op {
    fn parse_many(s: &str) -> Result<(Vec<Op>, &str), Errors<PointerOffset, char, &str>> {
        let mut programs_parser = sep_by(op_parser!(), char(','));
        programs_parser.easy_parse(s)
    }
}

struct Dancer<'a> {
    ops: &'a Vec<Op>,
    line: &'a mut Vec<char>,
    lookup: HashMap<char, usize>,
    cache: HashMap<Vec<char>, (Vec<char>, HashMap<char, usize>)>,
}

impl<'a> Dancer<'a> {
    fn new(ops: &'a Vec<Op>, line: &'a mut Vec<char>) -> Dancer<'a> {
        let lookup = line.iter().enumerate().map(|(idx, c)| (*c, idx)).collect();
        let cache = HashMap::new();
        Dancer {
            ops,
            line,
            lookup,
            cache,
        }
    }

    fn dance(&mut self, repeat: usize) -> () {
        use day_16::Op::*;
        let line_len = self.line.len();
        let mut cycles_run = 0;
        while cycles_run < repeat {
            if let Some((cached_result, cached_lookup)) =
                self.cache.get(self.line).map(|r| r.clone())
            {
                *self.line = cached_result;
                self.lookup = cached_lookup;
            } else {
                let init = self.line.clone();
                for op in self.ops.iter() {
                    match op {
                        &Spin(tail_len) => {
                            let v = {
                                let (head, tail) = self.line.split_at(line_len - tail_len);
                                let mut new_vec = Vec::with_capacity(line_len);
                                for (idx, c) in tail.iter().chain(head.iter()).enumerate() {
                                    self.lookup.insert(*c, idx);
                                    new_vec.push(*c);
                                }
                                new_vec
                            };
                            *self.line = v;
                        }
                        &Exchange(idx_1, idx_2) => {
                            self.line.swap(idx_1, idx_2);
                            self.lookup.insert(self.line[idx_1], idx_1);
                            self.lookup.insert(self.line[idx_2], idx_2);
                        }
                        &Partner(char_1, char_2) => {
                            if let Some(char_1_idx) = self.lookup.get(&char_1).map(|u| *u) {
                                if let Some(char_2_idx) = self.lookup.get(&char_2).map(|u| *u) {
                                    self.line.swap(char_1_idx, char_2_idx);
                                    self.lookup.insert(self.line[char_1_idx], char_1_idx);
                                    self.lookup.insert(self.line[char_2_idx], char_2_idx);
                                }
                            };
                        }
                    }
                }
                self.cache
                    .insert(init, (self.line.clone(), self.lookup.clone()));
            }
            cycles_run += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use day_16::*;
    use day_16::Op::*;

    fn test_input() -> Vec<char> {
        vec!['a', 'b', 'c', 'd', 'e']
    }

    #[test]
    fn test_op_mutate() {
        let mut line = test_input();
        let ops = vec![Spin(1), Exchange(3, 4), Partner('e', 'b')];
        let mut dancer = Dancer::new(&ops, &mut line);
        dancer.dance(1);
        assert_eq!(dancer.line, &vec!['b', 'a', 'e', 'd', 'c'])
    }

    #[test]
    fn test_parse_real_input() {
        let (ops, _) = Op::parse_many(DAY_16_INPUT).unwrap();
        assert!(ops.len() > 10);
    }

    #[test]
    fn find_solution_real_test() {
        let mut line = input_line();
        let (ops, _) = Op::parse_many(DAY_16_INPUT).unwrap();
        let mut dancer = Dancer::new(&ops, &mut line);
        dancer.dance(1);
        assert_eq!(
            dancer.line,
            &vec![
                'f', 'g', 'm', 'o', 'b', 'e', 'a', 'i', 'j', 'h', 'd', 'p', 'k', 'c', 'l', 'n'
            ]
        );
    }
}
