use std::ops::BitXor;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Mark(usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Idx(usize);

const HEX_HASH_ROUNDS: usize = 64;
const SIMPLE_HASH_ROUNDS: usize = 1;
const BITXOR_CHUNKSIZE: usize = 16;

pub fn solve_knot_hash(s: &str) -> Result<usize, String> {
    let v = knot_hash(s)?;
    if v.len() > 1 {
        Ok(v[0] * v[1])
    } else {
        Err(format!(
            "Result not long enough to find the answer to life: {:?}",
            v
        ))
    }
}

/// Returns the Knot Hash of the given string
///
/// # Example
/// ```
/// # use aoc_2017::day_10::*;
/// let input = "88,88,211,106,141,1,78,254,2,111,77,255,90,0,54,205";
/// assert_eq!(hex_knot_hash(input), Ok("e0387e2ad112b7c2ef344e44885fe4d8".to_string()));
/// ```
pub fn hex_knot_hash(s: &str) -> Result<String, String> {
    let as_u8_padded = as_u8_padded_vec(s);
    let as_hashed = generate_hashes(256, &as_u8_padded, HEX_HASH_ROUNDS)?;
    let dense_hash = to_dense_bitxored(&as_hashed, BITXOR_CHUNKSIZE);
    Ok(as_hexadecimal_string(&dense_hash))
}

fn as_u8_padded_vec(s: &str) -> Vec<usize> {
    [s.as_bytes(), &[17, 31, 73, 47, 23]]
        .concat()
        .into_iter()
        .map(|u| u as usize)
        .collect()
}

fn as_hexadecimal_string(v: &Vec<usize>) -> String {
    v.iter()
        .fold(String::with_capacity(v.len() * 2), |mut acc, next| {
            let as_hex = format!("{:02x}", next);
            acc.push_str(as_hex.as_str());
            acc
        })
}

fn knot_hash(s: &str) -> Result<Vec<usize>, String> {
    let v = s.split(",")
        .filter_map(|seg| seg.trim().parse().ok())
        .collect();
    generate_hashes(256, &v, SIMPLE_HASH_ROUNDS)
}

fn to_dense_bitxored<V>(v: &Vec<V>, chunksize: usize) -> Vec<V>
where
    V: BitXor<V, Output = V> + Copy,
{
    v.chunks(chunksize)
        .into_iter()
        .filter_map(|chunk| {
            if chunk.len() < chunksize || chunk.len() < 1 {
                None
            } else {
                let bitxored = chunk
                    .into_iter()
                    .skip(1)
                    .fold(chunk[0], |acc, next| acc ^ *next);
                Some(bitxored)
            }
        })
        .collect()
}

fn generate_hashes(
    mark_length: usize,
    lengths: &Vec<usize>,
    rounds: usize,
) -> Result<Vec<usize>, String> {
    let mut marks = generate_marks(mark_length);
    reverse_at_lengths(&mut marks, lengths, rounds)?;
    Ok(marks.iter().map(|&Mark(v)| v).collect())
}

fn generate_marks(length: usize) -> Vec<Mark> {
    (0..length).map(|i| Mark(i)).collect()
}

fn reverse_at_lengths(
    v: &mut Vec<Mark>,
    lengths: &Vec<usize>,
    rounds: usize,
) -> Result<(), String> {
    let v_len = v.len();
    let lengths_len = lengths.len();
    let mut current_idx = Idx(0);
    let mut current_skip = 0;

    lengths
        .iter()
        .cycle()
        .take(rounds * lengths_len)
        .fold(Ok(()), |acc, length| {
            match acc {
                Ok(_) => {
                    let r = reverse_at_index(v, current_idx, *length);
                    if let Ok(_) = r {
                        if current_idx.0 + current_skip + length >= v.len() {
                            // assuming v is [0, 1, 2, 3, 4] -> v.len() is 5
                            // current_idx = 2, current_skip = 1, length = 3
                            // next idx needs to be at 1 (index also 1)
                            // unwrapped_idx = current_idx + current_skip + length
                            //               = 2 + 1 + 3 = 6
                            // next index = unwrapped_idx - v.len()
                            //            = 6 - 5 = 1
                            let wrapped_next_idx = {
                                let unwrapped_next_idx = current_idx.0 + current_skip + length;
                                let factors = unwrapped_next_idx / v_len;
                                unwrapped_next_idx - factors * v_len
                            };
                            current_idx = Idx(wrapped_next_idx);
                        } else {
                            // assuming v is [0, 1, 2, 3, 4] -> v.len() is 5
                            // current_idx = 0, current_skip = 1, length = 3
                            // next idx needs to be at 4 (index also 4)
                            // next_idx = current_idx + current_skip + length
                            //          = 0 + 1 + 3 = 4
                            current_idx = Idx(current_idx.0 + current_skip + length);
                        }
                        current_skip += 1;
                    }
                    r
                }
                e => e,
            }
        })
}

fn reverse_at_index(v: &mut Vec<Mark>, idx: Idx, length: usize) -> Result<(), String> {
    let v_length = v.len();
    if length > v_length {
        Err(format!(
            "Vector was of length [{}] but reverse length was longer [{}]",
            v.len(),
            length
        ))
    } else {
        if (idx.0 + length) > v_length {
            // assuming idx = 2, length = 4, v is [0, 1, 2, 3, 4], so v.len() = 5
            // idx + length = 6
            // our 1st slice will need to be:
            //   v[idx .. v.len()] -> v[2 .. 5] -> [2, 3, 4]
            // our 2nd slice will need to be:
            //   v[0 .. (idx + length - v.len())] -? v[0 .. (2 + 4 - 5)] -> v[0 .. 1] -> [0]
            let wrapped_around_end = idx.0 + length - v_length;
            let mut combined_vec = [&v[idx.0..v_length], &v[0..wrapped_around_end]].concat();
            // create a chained iterator that goes through the indices in proper
            // sequence
            let indices_iter = (idx.0..v_length).chain(0..wrapped_around_end);
            // start replacing the original elements in v with our reversed, based on index
            for i in indices_iter {
                // pop from the end of our combined vec, which will essentially be
                // the same as  reversing
                if let Some(last) = combined_vec.pop() {
                    v[i] = last;
                };
            }
        } else {
            // straightforward
            let slice = &mut v[idx.0..idx.0 + length];
            slice.reverse();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use day_10::*;

    #[test]
    fn generate_hashes_test() {
        let input_lengths = vec![3, 4, 1, 5];
        let r = generate_hashes(5, &input_lengths, SIMPLE_HASH_ROUNDS).unwrap();
        assert_eq!(r, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn solve_knot_hash_test() {
        let r = solve_knot_hash(DAY_10_INPUT).unwrap();
        assert_eq!(r, 11375);
    }

    #[test]
    fn hex_knot_hash_test() {
        let r1 = hex_knot_hash("").unwrap();
        let r2 = hex_knot_hash("AoC 2017").unwrap();
        let r3 = hex_knot_hash("1,2,3").unwrap();
        let r4 = hex_knot_hash("1,2,4").unwrap();
        assert_eq!(r1, "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(r2, "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(r3, "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(r4, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn as_hexadecimal_string_test() {
        assert_eq!(
            as_hexadecimal_string(&vec![64, 7, 255]),
            "4007ff".to_string()
        );
    }

    #[test]
    fn to_dense_bitxored_test() {
        let input = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!(to_dense_bitxored(&input, 16)[0], 64);
    }
}

pub const DAY_10_INPUT: &'static str = "88,88,211,106,141,1,78,254,2,111,77,255,90,0,54,205";
