use std::collections::*;
use std::collections::hash_map::Entry;
use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;
use std::error::Error;

const DAY_12_INPUT: &str = include_str!("../data/day_12_input");

pub fn run() -> Result<(), Box<Error>> {
    println!("*** Day 12: Digital Plumber ***");
    println!("Input: {}", DAY_12_INPUT);
    let programs_in_group = find_programs_in_group(DAY_12_INPUT, ProgramId(0))?;
    println!("Solution1: {}\n", programs_in_group.routes.len());
    println!("Solution2: {}\n", find_all_groups(DAY_12_INPUT)?.len());
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct ProgramId(usize);

#[derive(PartialEq, Eq, Debug, Hash)]
struct Pipe {
    id: ProgramId,
    links: Vec<ProgramId>,
}

#[derive(Debug)]
struct Pipes {
    topology: HashMap<ProgramId, Vec<ProgramId>>,
}

fn find_programs_in_group(
    input: &str,
    target_id: ProgramId,
) -> Result<Group, Errors<PointerOffset, char, &str>> {
    let (pipes, _) = Pipes::parse(input)?;
    Ok(group(&pipes, target_id))
}

fn find_all_groups(input: &str) -> Result<Vec<Group>, Errors<PointerOffset, char, &str>> {
    let (pipes, _) = Pipes::parse(input)?;
    let groups: Vec<Group> = pipes
        .topology
        .iter()
        .fold(Vec::new(), |mut acc, (prog_id, _)| {
            let prog_id_already_in_a_group = acc.iter()
                .find(|group| group.routes.contains_key(prog_id))
                .is_some();
            if prog_id_already_in_a_group {
                acc
            } else {
                let id_group = group(&pipes, *prog_id);
                acc.push(id_group);
                acc
            }
        });
    Ok(groups)
}

impl Pipes {
    fn parse(s: &str) -> Result<(Pipes, &str), Errors<PointerOffset, char, &str>> {
        let (programs, remainder) = parse_pipes(s)?;
        let programs_len = programs.len();
        let topology = programs.into_iter().fold(
            HashMap::with_capacity(programs_len),
            |mut acc, next| {
                acc.insert(next.id, next.links);
                acc
            },
        );
        Ok((Pipes { topology }, remainder))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Group {
    id: ProgramId,
    routes: HashMap<ProgramId, Vec<ProgramId>>,
}

fn group(pipes: &Pipes, id: ProgramId) -> Group {
    let mut group = Group {
        id: id,
        routes: HashMap::new(),
    };
    let route = Vec::with_capacity(pipes.topology.len());

    // Could probably do this nicer with a BFS, but tracking routes
    // and handling the bi-directional links gets a bit tricky...
    fn explore_link(
        pipes: &Pipes,
        current: ProgramId,
        mut seen_in_traversal: HashSet<ProgramId>,
        previous_route: &Vec<ProgramId>,
        group: &mut Group,
    ) -> () {
        let current_route = {
            let mut cloned = previous_route.clone();
            cloned.push(current);
            cloned
        };
        if let Some(next_ids) = pipes.topology.get(&current) {
            let unexplored_in_current_traversal: Vec<_> = next_ids
                .iter()
                .filter(|next_id| !seen_in_traversal.contains(next_id))
                .collect();
            seen_in_traversal.insert(current);
            for unexplored_id in unexplored_in_current_traversal {
                explore_link(
                    pipes,
                    *unexplored_id,
                    seen_in_traversal.clone(),
                    &current_route,
                    group,
                );
            }
        }

        let reversed_route = {
            let mut cloned = current_route.clone();
            cloned.reverse();
            cloned
        };
        match group.routes.entry(current) {
            Entry::Occupied(mut entry) => {
                let current = entry.get_mut();
                if current.len() > reversed_route.len() {
                    *current = reversed_route;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(reversed_route);
            }
        }
    }

    explore_link(pipes, id, HashSet::new(), &route, &mut group);
    group
}

macro_rules! id_parser {
        () => {
            many1::<String, _>(digit()).and_then(|s| {
                s.parse::<usize>().map(|u| ProgramId(u))
            })
        }
}

macro_rules! tabs_or_spaces {
    () => {
        many::<Vec<char>, _>(try(char(' ')).or(char('\t')))
    }
}

macro_rules! program_parser {
    () => {
        {
            let link_delimiter_parser = string("<->");
            let ids_parser = sep_by(
                id_parser!(),
                tabs_or_spaces!()
                    .with(token(',')
                        .skip(tabs_or_spaces!())
                    )
                );
            id_parser!()
                .skip(tabs_or_spaces!())
                .skip(link_delimiter_parser)
                .skip(tabs_or_spaces!())
                .and(ids_parser)
                .map(|(id, links)| Pipe { id, links })
        }
    };
}

fn parse_pipes(s: &str) -> Result<(Vec<Pipe>, &str), Errors<PointerOffset, char, &str>> {
    let mut programs_parser = skip_many(newline()).with(sep_by(program_parser!(), spaces()));
    programs_parser.easy_parse(s)
}

#[cfg(test)]
mod tests {
    use day_12::*;

    const TEST_INPUT: &str = r#"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#;

    #[test]
    fn program_parser_simple_test() {
        let mut p = program_parser!();
        let input = "0 <-> 2";
        let (r, _) = p.easy_parse(input).unwrap();
        assert_eq!(
            r,
            Pipe {
                id: ProgramId(0),
                links: vec![ProgramId(2)],
            }
        );
    }

    #[test]
    fn program_parser_multilinks_test() {
        let mut p = program_parser!();
        let input = "4 <-> 2, 3, 6";
        let (r, _) = p.easy_parse(input).unwrap();
        assert_eq!(
            r,
            Pipe {
                id: ProgramId(4),
                links: vec![ProgramId(2), ProgramId(3), ProgramId(6)],
            }
        );
    }

    #[test]
    fn programs_parse_test() {
        let (programs, _) = parse_pipes(TEST_INPUT).unwrap();
        assert_eq!(programs.len(), 7);
        assert_eq!(
            programs,
            vec![
                Pipe {
                    id: ProgramId(0),
                    links: vec![ProgramId(2)],
                },
                Pipe {
                    id: ProgramId(1),
                    links: vec![ProgramId(1)],
                },
                Pipe {
                    id: ProgramId(2),
                    links: vec![ProgramId(0), ProgramId(3), ProgramId(4)],
                },
                Pipe {
                    id: ProgramId(3),
                    links: vec![ProgramId(2), ProgramId(4)],
                },
                Pipe {
                    id: ProgramId(4),
                    links: vec![ProgramId(2), ProgramId(3), ProgramId(6)],
                },
                Pipe {
                    id: ProgramId(5),
                    links: vec![ProgramId(6)],
                },
                Pipe {
                    id: ProgramId(6),
                    links: vec![ProgramId(4), ProgramId(5)],
                },
            ]
        )
    }

    #[test]
    fn pipes_parse_test() {
        let (pipes, _) = Pipes::parse(TEST_INPUT).unwrap();
        assert_eq!(
            pipes.topology.get(&ProgramId(4)),
            Some(&vec![ProgramId(2), ProgramId(3), ProgramId(6)])
        );
    }

    #[test]
    fn group_test() {
        let (pipes, _) = Pipes::parse(TEST_INPUT).unwrap();
        let group = group(&pipes, ProgramId(0));

        assert_eq!(
            group,
            Group {
                id: ProgramId(0),
                routes: hashmap!{
                    ProgramId(0) => vec![ProgramId(0)],
                    ProgramId(6) => vec![ProgramId(6), ProgramId(4), ProgramId(2), ProgramId(0)],
                    ProgramId(3) => vec![ProgramId(3), ProgramId(2), ProgramId(0)],
                    ProgramId(4) => vec![ProgramId(4), ProgramId(2), ProgramId(0)],
                    ProgramId(5) => vec![
                        ProgramId(5),
                        ProgramId(6),
                        ProgramId(4),
                        ProgramId(2),
                        ProgramId(0),
                    ],
                    ProgramId(2) => vec![ProgramId(2), ProgramId(0)]
                },
            }
        );
    }

    #[test]
    fn parse_real_input_test() {
        let (pipes, _) = Pipes::parse(DAY_12_INPUT).unwrap();
        assert!(pipes.topology.len() > 10);
    }

    #[test]
    fn first_half_real_input_test() {
        let programs_in_group = find_programs_in_group(DAY_12_INPUT, ProgramId(0)).unwrap();
        assert_eq!(programs_in_group.routes.len(), 169);
    }

    #[test]
    fn find_all_groups_real_test() {
        let groups = find_all_groups(DAY_12_INPUT).unwrap();
        assert_eq!(groups.len(), 179);
    }
}
