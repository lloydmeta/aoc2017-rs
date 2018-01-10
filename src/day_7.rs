use regex::*;
use std::collections::{HashMap, HashSet};

const NAME_GROUP: &str = "name";
const WEIGHT_GROUP: &str = "weight";
const HOLDING_UP_GROUP: &str = "holding_up";
const DAY_7_INPUT: &str = include_str!("../data/day_7_input");

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 7: Recursive Circus ***");
    println!("Input: {}", DAY_7_INPUT);
    let tree = Node::from_str(DAY_7_INPUT)?;
    println!("Solution 1: {:?}\n", tree.name);
    let with_kid_weights = NodeWithChildrenWeight::build(&tree);
    println!(
        "Solution 2: {:?}\n",
        with_kid_weights.smallest_rebalanced_children_weight()
    );
    Ok(())
}

lazy_static! {
    static ref ENTRIES_MATCHER: Regex = {
        Regex::new(
            &format!(
                r#"(?P<{}>\w+)\s\((?P<{}>\d+)\)(?:\s->\s(?P<{}>.*))?"#,
                NAME_GROUP,
                WEIGHT_GROUP,
                HOLDING_UP_GROUP
            )
        ).unwrap()
    };
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Name(String);

#[derive(PartialEq, Eq, Hash, Debug)]
struct NodeEntry {
    name: Name,
    weight: usize,
    holding_up: Vec<Name>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    name: Name,
    weight: usize,
    children: Vec<Node>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct NodeWithChildrenWeight<'a> {
    node: &'a Node,
    children_with_weights: Vec<NodeWithChildrenWeight<'a>>,
    children_weight: usize,
}

impl<'a> NodeWithChildrenWeight<'a> {
    fn build(node: &'a Node) -> NodeWithChildrenWeight<'a> {
        let children_with_weights: Vec<_> = node.children
            .iter()
            .map(NodeWithChildrenWeight::build)
            .collect();
        let children_weight = children_with_weights.iter().fold(
            0,
            |acc, child_with_children_weights| {
                acc + child_with_children_weights.children_weight
                    + child_with_children_weights.node.weight
            },
        );
        NodeWithChildrenWeight {
            node: node,
            children_with_weights: children_with_weights,
            children_weight: children_weight,
        }
    }

    fn total_weight(&self) -> usize {
        self.children_weight + self.node.weight
    }

    fn smallest_rebalanced_children_weight(&self) -> Result<isize, &'static str> {
        let children_smallest = self.children_with_weights.iter().fold(
            Err("No need for rebalancing"),
            |acc, next| {
                let next_smallest_r = next.smallest_rebalanced_children_weight();
                match (acc, next_smallest_r) {
                    (Err(_), r) => r,
                    (ok @ Ok(_), Err(_)) => ok,
                    (Ok(last_smallest), Ok(next_smallest)) => if next_smallest < last_smallest {
                        Ok(next_smallest)
                    } else {
                        acc
                    },
                }
            },
        );
        children_smallest.or(self.rebalanced_children_weight())
    }

    fn rebalanced_children_weight(&self) -> Result<isize, &'static str> {
        let kid_weights = self.children_with_weights.as_slice();
        if kid_weights.len() <= 1 {
            Err("There are 1 or less children weights. No need for rebalancing")
        } else {
            // Children nodes deduplicated by total weight.
            let deduped_children_nodes_with_weights = {
                let mut t: Vec<_> = self.children_with_weights
                    .iter()
                    .map(|cww| (cww.node, cww.total_weight()))
                    .collect();
                t.sort_by(|&(_, cww1), &(_, cww2)| cww1.cmp(&cww2));
                t.dedup_by(|&mut (_, cww1), &mut (_, cww2)| cww1 == cww2);
                t
            };
            // Children nodes deduplicated by total weight, including count of children nodes
            // that share the same weight
            let deduped_children_nodes_with_weight_and_counts = {
                deduped_children_nodes_with_weights
                    .iter()
                    .map(|&(child, total_weight)| {
                        let count = kid_weights
                            .iter()
                            .filter(|k| k.total_weight() == total_weight)
                            .count();
                        ((child, total_weight), count)
                    })
            };
            let children_with_unique_weights: Vec<_> =
                deduped_children_nodes_with_weight_and_counts
                    .filter(|&(_, count)| count == 1)
                    .collect();
            if children_with_unique_weights.len() != 1 {
                Err("Could not find a singular child node in this tree for adjustment.")
            } else {
                let (odd_node, odd_node_total_weight) = children_with_unique_weights[0].0;
                // Retrieve another node where the total weight (w/ kids) is not the same, then diff
                if let Some(&(_, other_total_weight)) = deduped_children_nodes_with_weights
                    .iter()
                    .find(|&&(_, total_weight)| total_weight != odd_node_total_weight)
                {
                    let diff = other_total_weight as isize - odd_node_total_weight as isize;
                    Ok(odd_node.weight as isize + diff)
                } else {
                    Err("No other weights exist.")
                }
            }
        }
    }
}

impl Node {
    fn from_str(s: &str) -> Result<Node, &'static str> {
        let entries = NodeEntry::parse(s);
        Node::from_entries(entries)
    }

    fn from_entries(entries: Vec<NodeEntry>) -> Result<Node, &'static str> {
        if let Some(root_entry) = find_root(&entries) {
            // O(1) access to entries by name.
            let mut names_to_entries = HashMap::with_capacity(entries.len());
            for entry in entries.iter() {
                names_to_entries.insert(&entry.name, entry);
            }
            Ok(Node::build(root_entry, &names_to_entries))
        } else {
            Err("No root entry found ")
        }
    }

    fn build(n: &NodeEntry, names_to_entries: &HashMap<&Name, &NodeEntry>) -> Node {
        let children_nodes: Vec<_> = n.holding_up
            .iter()
            .filter_map(|entry_name| {
                names_to_entries
                    .get(entry_name)
                    .map(|node| Node::build(node, names_to_entries))
            })
            .collect();
        Node {
            name: n.name.clone(),
            weight: n.weight,
            children: children_nodes,
        }
    }
}

fn find_root(entries: &Vec<NodeEntry>) -> Option<&NodeEntry> {
    let held_up = entries.iter().fold(HashSet::new(), |mut acc, p| {
        for o in p.holding_up.iter() {
            acc.insert(o);
        }
        acc
    });
    entries.iter().find(|p| !held_up.contains(&p.name))
}

impl NodeEntry {
    fn parse(to_parse: &str) -> Vec<NodeEntry> {
        // Not sure if we can get multiple matches in a single string.
        to_parse
            .trim()
            .split("\n")
            .filter_map(|s| {
                ENTRIES_MATCHER.captures(s).and_then(|captures| {
                    match (
                        captures.name(NAME_GROUP),
                        captures
                            .name(WEIGHT_GROUP)
                            .and_then(|ws| ws.as_str().parse().ok()),
                    ) {
                        (Some(name), Some(weight)) => {
                            let others = captures
                                .name(HOLDING_UP_GROUP)
                                .map(|c| {
                                    c.as_str()
                                        .split(",")
                                        .map(|s| Name(s.trim().to_string()))
                                        .collect()
                                })
                                .unwrap_or_else(|| vec![]);
                            Some(NodeEntry {
                                name: Name(name.as_str().to_string()),
                                weight: weight,
                                holding_up: others,
                            })
                        }
                        _ => None,
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use day_7::*;

    const TEST_INPUT: &str = r#"
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;

    #[test]
    fn entries_matcher_regex_test() {
        let parsed = NodeEntry::parse(DAY_7_INPUT);
        assert!(parsed.len() > 0);
        for p in parsed {
            println!("{:?}", p);
        }
    }

    #[test]
    fn find_root_test() {
        let parsed = NodeEntry::parse(TEST_INPUT);
        let p = find_root(&parsed).unwrap();
        assert_eq!(p.name, Name("tknk".to_string()));
    }

    #[test]
    fn node_parse_test() {
        let tree = Node::from_str(TEST_INPUT).unwrap();
        println!("{:?}", tree);
        assert_eq!(tree.name, Name("tknk".to_string()));
    }

    #[test]
    fn smallest_rebalanced_children_weight_dry_test() {
        let tree = Node::from_str(TEST_INPUT).unwrap();
        let with_kids_weights = NodeWithChildrenWeight::build(&tree);
        let rebalance = with_kids_weights.smallest_rebalanced_children_weight();
        assert_eq!(rebalance, Ok(60));
    }

    #[test]
    fn smallest_rebalanced_children_weight_real_test() {
        let tree = Node::from_str(DAY_7_INPUT).unwrap();
        let with_kids_weights = NodeWithChildrenWeight::build(&tree);
        let rebalance = with_kids_weights.smallest_rebalanced_children_weight();
        assert_eq!(rebalance, Ok(2310));
    }
}
