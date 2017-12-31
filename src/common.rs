// Can't be arsed to figure out the return types for these horrors.
macro_rules! number_parser {
    ($t: ty) => {
        token('-')
        .with(many1::<String, _>(digit()).and_then(|s| s.parse::<$t>().map(|v| -v)))
        .or(many1::<String, _>(digit()).and_then(|s| s.parse::<$t>()));
    }
}

macro_rules! pos_number_parser {
    ($t: ty) => {
        many1::<String, _>(digit()).and_then(|s| s.parse::<$t>());
    }
}

macro_rules! tabs_or_spaces {
    () => {
        many::<Vec<char>, _>(try(char(' ')).or(char('\t')))
    }
}