const GEN_A_FACTOR: u64 = 16807;
const GEN_B_FACTOR: u64 = 48271;

const DIVIDER: u64 = 2147483647;

const DAY_15_INPUT: GeneratedValues = GeneratedValues { a: 679, b: 771 };

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 14: Disk Defragmentation ***");
    println!("Input: {:?}", DAY_15_INPUT);
    println!("Solution1: {:?}", find_matching(DAY_15_INPUT));
    println!("Solution2: {:?}", find_choosey(DAY_15_INPUT));
    Ok(())
}

fn find_matching(init: GeneratedValues) -> usize {
    init.simple_iter()
        .take(40_000_000)
        .filter(|generated| lower_16_bits_match(&generated))
        .count()
}

fn find_choosey(init: GeneratedValues) -> usize {
    init.choosey_iter(|u| u % 4 == 0, |u| u % 8 == 0)
        .take(5_000_000)
        .filter(|generated| lower_16_bits_match(&generated))
        .count()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct GeneratedValues {
    a: u64,
    b: u64,
}

struct SimpleGeneratedValuesIterator {
    values: GeneratedValues,
}

struct ComplexGeneratedValuesIterator<F1, F2>
where
    F1: Fn(u64) -> bool,
    F2: Fn(u64) -> bool,
{
    values: GeneratedValues,
    f_a: F1,
    f_b: F2,
}

impl GeneratedValues {
    fn simple_iter(&self) -> SimpleGeneratedValuesIterator {
        SimpleGeneratedValuesIterator { values: *self }
    }

    fn choosey_iter<F1, F2>(&self, f_a: F1, f_b: F2) -> ComplexGeneratedValuesIterator<F1, F2>
    where
        F1: Fn(u64) -> bool,
        F2: Fn(u64) -> bool,
    {
        ComplexGeneratedValuesIterator {
            values: *self,
            f_a: f_a,
            f_b: f_b,
        }
    }
}

fn lower_16_bits_match(generated: &GeneratedValues) -> bool {
    let bixored = generated.a ^ generated.b;
    (bixored << 48) == 0
}

impl Iterator for SimpleGeneratedValuesIterator {
    type Item = GeneratedValues;

    fn next(&mut self) -> Option<GeneratedValues> {
        let next_a = (GEN_A_FACTOR * self.values.a) % DIVIDER;
        let next_b = (GEN_B_FACTOR * self.values.b) % DIVIDER;
        self.values.a = next_a;
        self.values.b = next_b;
        Some(self.values)
    }
}

impl<F1, F2> Iterator for ComplexGeneratedValuesIterator<F1, F2>
where
    F1: Fn(u64) -> bool,
    F2: Fn(u64) -> bool,
{
    type Item = GeneratedValues;
    fn next(&mut self) -> Option<GeneratedValues> {
        let current_b = self.values.b;
        let next_a = self.values
            .simple_iter()
            .skip_while(|g| !(self.f_a)(g.a))
            .next()
            .map(|g| g.a);
        // reset b of values
        self.values.b = current_b;
        let next_b = self.values
            .simple_iter()
            .skip_while(|g| !(self.f_b)(g.b))
            .next()
            .map(|g| g.b);
        match (next_a, next_b) {
            (Some(a), Some(b)) => {
                self.values.a = a;
                self.values.b = b;
                Some(GeneratedValues { a, b })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use day_15::*;

    const TEST_INPUT: GeneratedValues = GeneratedValues { a: 65, b: 8921 };

    #[test]
    fn generator_test() {
        let mut first = TEST_INPUT.simple_iter();
        let next_1 = first.next().unwrap();
        let next_2 = first.next().unwrap();
        let next_3 = first.next().unwrap();

        assert_eq!(
            next_1,
            GeneratedValues {
                a: 1092455,
                b: 430625591,
            }
        );

        assert_eq!(
            next_2,
            GeneratedValues {
                a: 1181022009,
                b: 1233683848,
            }
        );

        assert_eq!(
            next_3,
            GeneratedValues {
                a: 245556042,
                b: 1431495498,
            }
        );
    }

    #[test]
    fn find_matching_test() {
        let r = find_matching(TEST_INPUT);
        assert_eq!(r, 588);
    }

    #[test]
    fn find_matching_part_1_real_test() {
        let r = find_matching(DAY_15_INPUT);
        assert_eq!(r, 626);
    }

    #[test]
    fn find_choosey_test() {
        let r = find_choosey(TEST_INPUT);
        assert_eq!(r, 309);
    }

    #[test]
    fn find_choosey_real_test() {
        let r = find_choosey(DAY_15_INPUT);
        assert_eq!(r, 306);
    }

    #[test]
    fn lower_16_bits_match_test() {
        assert!(lower_16_bits_match(&GeneratedValues {
            a: 245556042,
            b: 1431495498,
        }))
    }
}
