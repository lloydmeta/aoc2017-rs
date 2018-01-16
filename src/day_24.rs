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
}

#[cfg(test)]
mod tests {

    use day_24::*;

    #[test]
    fn components_parse_many_test() {
        let parsed = Component::parse_many(DAY_24_INPUT);
        assert!(parsed.is_ok())
    }
}
