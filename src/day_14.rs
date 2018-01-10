use common::hex_knot_hash;
use std::collections::HashMap;
use std::usize;

const DAY_14_INPUT: &'static str = include_str!("../data/day_14_input");

const MAX_ROWS: usize = 128;
const OCCUPIED_CHAR: char = '1';

pub fn run() -> Result<(), String> {
    println!("*** Day 14: Disk Defragmentation ***");
    println!("Input: {}", DAY_14_INPUT);
    let binary_disk_usage_repr = to_binary_repr(DAY_14_INPUT)?;
    println!("Solution1: {:?}\n", count_1s(&binary_disk_usage_repr));
    let discovered_regions = discover_regions(&binary_disk_usage_repr);
    println!(
        "Solution2: {:?}\n",
        discovered_regions.regions_to_coords.len()
    );
    Ok(())
}

fn to_binary_repr(s: &str) -> Result<Vec<Vec<char>>, String> {
    let knotted_hex_rows = (0..MAX_ROWS)
        .map(|i| {
            let row_input = format!("{}-{}", s, i);
            hex_knot_hash(&row_input)
        })
        .fold(
            Ok(Vec::with_capacity(MAX_ROWS)),
            |acc: Result<Vec<String>, String>, knot_hash_result| match (acc, knot_hash_result) {
                (Ok(mut v), Ok(knot_hash)) => {
                    v.push(knot_hash);
                    Ok(v)
                }
                (moved_acc, _) => moved_acc,
            },
        )?;
    let as_binary_rows: Vec<_> = knotted_hex_rows
        .iter()
        .map(|row| {
            row.chars()
                .filter_map(|c| {
                    usize::from_str_radix(c.to_string().as_str(), 16)
                        .ok()
                        .map(|as_usize| {
                            let s = format!("{:04b}", as_usize);
                            s
                        })
                })
                .flat_map(|s| {
                    let v: Vec<char> = s.chars().collect();
                    v
                })
                .collect()
        })
        .collect();
    Ok(as_binary_rows)
}

fn count_1s(v: &Vec<Vec<char>>) -> usize {
    v.iter().fold(0, |acc, s| {
        s.iter().fold(acc, |inner_acc, next_char| {
            if *next_char == OCCUPIED_CHAR {
                inner_acc + 1
            } else {
                inner_acc
            }
        })
    })
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coords {
    i: usize,
    j: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
struct Region(usize);

// Two lookups that we need to keep in sync, but is worth
// the hassle...
struct RegionDiscovery {
    coords_to_regions: HashMap<Coords, Region>,
    regions_to_coords: HashMap<Region, Vec<Coords>>,
}

fn discover_regions(v: &Vec<Vec<char>>) -> RegionDiscovery {
    let row_count = v.len();
    let col_count = v.iter().fold(row_count, |acc, row| {
        if row.len() < acc {
            row.len()
        } else {
            acc
        }
    });
    let mut region_discovery = RegionDiscovery {
        coords_to_regions: HashMap::with_capacity(row_count * col_count),
        regions_to_coords: HashMap::with_capacity(row_count),
    };
    let mut region_id = Region(0);
    for i in 0..row_count {
        for j in 0..col_count {
            if let Some(c) = v.get(i).and_then(|r| r.get(j)) {
                if *c == OCCUPIED_CHAR {
                    let maybe_left_coords = {
                        if j > 0 {
                            Some(Coords { i: i, j: j - 1 })
                        } else {
                            None
                        }
                    };
                    let maybe_up_coords = {
                        if i > 0 {
                            Some(Coords { i: i - 1, j: j })
                        } else {
                            None
                        }
                    };
                    let maybe_left_region = maybe_left_coords
                        .and_then(|left_coords| {
                            region_discovery.coords_to_regions.get(&left_coords)
                        })
                        .map(|r| *r);
                    let maybe_up_region = maybe_up_coords
                        .and_then(|up_coords| region_discovery.coords_to_regions.get(&up_coords))
                        .map(|r| *r);
                    let lowest_region_id = match (maybe_left_region, maybe_up_region) {
                        (Some(left_region), Some(up_region)) => if left_region < up_region {
                            left_region
                        } else {
                            up_region
                        },
                        (Some(left_region), None) => left_region,
                        (None, Some(up_region)) => up_region,
                        _ => {
                            let r = region_id;
                            region_id.0 += 1;
                            r
                        }
                    };
                    let regions_to_update: Vec<Region> = vec![maybe_left_region, maybe_up_region]
                        .iter()
                        .filter_map(|maybe| *maybe)
                        .collect();
                    region_discovery.add_coords_to_region(
                        Coords { i, j },
                        lowest_region_id,
                        regions_to_update,
                    );
                }
            }
        }
    }
    region_discovery
}

impl RegionDiscovery {
    // Adds a coord to a region; also in charge of keeping things in sync
    // when updating Coords already assigned a region to a new region wehn
    // we discover that the coords are all connected.
    fn add_coords_to_region(
        &mut self,
        coords: Coords,
        region: Region,
        regions_to_convert: Vec<Region>,
    ) -> () {
        let mut coords_in_target_regions: Vec<Coords> = regions_to_convert
            .iter()
            .filter_map(|target_region| self.regions_to_coords.remove(target_region))
            .flat_map(|v| v)
            .collect();
        // update coords_to_region
        self.coords_to_regions.insert(coords, region);
        for coords_to_update in coords_in_target_regions.iter() {
            self.coords_to_regions.insert(*coords_to_update, region);
        }
        // update region_to_coords
        let region_to_coords_entry = self.regions_to_coords.entry(region).or_insert(vec![]);
        region_to_coords_entry.push(coords);
        region_to_coords_entry.append(&mut coords_in_target_regions);
    }
}

#[cfg(test)]
mod tests {
    use day_14::*;

    const TEST_INPUT: &'static str = "flqrgnkx";

    #[test]
    fn to_binary_repr_test() {
        let r = to_binary_repr(TEST_INPUT).unwrap();
        assert_eq!(count_1s(&r), 8108);
    }

    #[test]
    fn to_binary_repr_real_test() {
        let r = to_binary_repr(DAY_14_INPUT).unwrap();
        assert_eq!(count_1s(&r), 8140);
    }

    #[test]
    fn regions_to_coords_test() {
        let r = to_binary_repr(TEST_INPUT).unwrap();
        let regions = discover_regions(&r);
        assert_eq!(regions.regions_to_coords.len(), 1242);
    }

    #[test]
    fn regions_to_coords_real_test() {
        let r = to_binary_repr(DAY_14_INPUT).unwrap();
        let regions = discover_regions(&r);
        assert_eq!(regions.regions_to_coords.len(), 1182);
    }
}
