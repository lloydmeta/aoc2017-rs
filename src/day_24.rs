use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

const DAY_24_INPUT: &'static str = include_str!("../data/day_24_input");

struct Component {
    p1: usize,
    p2: usize,
}

impl Component {
    fn parse_many(s: &str) -> Result<Vec<Component>, Errors<PointerOffset, char, &str>> {
        let mut parser = sep_by(
            pos_number_parser!(usize)
                .skip(char('/'))
                .and(pos_number_parser!(usize))
                .map(|(p1, p2)| Component { p1, p2 }),
            spaces(),
        );
        let (s, _) = parser.easy_parse(s)?;
        Ok(s)
    }

    fn is_valid_chain(components: &[Component]) -> bool {
        fn go(current: usize, xs: &[Component]) -> bool {
            /*
             * Can probably collapse these into a single boolean expression, but
             * readability would suffer, so ... LLVM do your thang
             */
            if let Some(next) = xs.first() {
                // find the first port in next that matches current_left
                if current == next.p1 {
                    go(next.p2, &xs[1..])
                } else if current == next.p2 {
                    go(next.p1, &xs[1..])
                } else {
                    false // no match
                }
            } else {
                true // reached the end, nothing to do, so we're good
            }
        }
        if let Some(first) = components.first() {
            // try both sides
            go(first.p1, &components[1..]) || go(first.p2, &components[1..])
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use day_24::*;

    #[test]
    fn components_parse_many_test() {
        let parsed = Component::parse_many(DAY_24_INPUT);
        assert!(parsed.is_ok())
    }

    #[test]
    fn is_valid_chain_test() {
        let chain_1 = vec![Component { p1: 0, p2: 1 }];
        assert!(Component::is_valid_chain(&chain_1));
        let chain_2 = vec![Component { p1: 0, p2: 1 }, Component { p1: 0, p2: 1 }];
        assert!(Component::is_valid_chain(&chain_2));
        let chain_3 = vec![Component { p1: 0, p2: 1 }, Component { p1: 10, p2: 1 }];
        assert!(Component::is_valid_chain(&chain_3));
        let chain_4 = vec![
            Component { p1: 0, p2: 1 },
            Component { p1: 10, p2: 1 },
            Component { p1: 9, p2: 10 },
        ];
        assert!(Component::is_valid_chain(&chain_4));
        let chain_5 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 4 },
        ];
        assert!(Component::is_valid_chain(&chain_5));
        let chain_6 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 5 },
        ];
        assert!(Component::is_valid_chain(&chain_6));
        let chain_7 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 2, p2: 2 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 5 },
        ];
        assert!(Component::is_valid_chain(&chain_7));
        let chain_8 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 2, p2: 2 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 4 },
        ];
        assert!(Component::is_valid_chain(&chain_8));
        let chain_9 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 3, p2: 4 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 4 },
        ];
        assert!(!Component::is_valid_chain(&chain_9));
        let chain_10 = vec![
            Component { p1: 0, p2: 2 },
            Component { p1: 2, p2: 4 },
            Component { p1: 2, p2: 3 },
            Component { p1: 3, p2: 4 },
        ];
        assert!(!Component::is_valid_chain(&chain_10));
        let chain_11 = vec![
            Component { p1: 2, p2: 0 },
            Component { p1: 2, p2: 0 },
            Component { p1: 0, p2: 3 },
            Component { p1: 5, p2: 4 },
        ];
        assert!(!Component::is_valid_chain(&chain_11));
    }
}
