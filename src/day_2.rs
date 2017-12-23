/// Returns the checksum of a matrix
///
/// # Examples
///
/// ```
/// # use aoc_2017::day_2::*;
/// let m =
/// "5\t1\t9\t5
/// 7\t5\t3
/// 2\t4\t6\t8";
/// assert_eq!(checksum(&m), 18);
/// ```
pub fn checksum(s: &str) -> isize {
    let m = string_to_matrix(s);
    matrix_checksum(&m)
}

fn matrix_checksum(matrix: &Vec<Vec<isize>>) -> isize {
    let max_mins = matrix.into_iter().map(|v| {
        v.into_iter().fold(None, |acc, next| match acc {
            Some((min, max)) => if next < min {
                Some((next, max))
            } else if next > max {
                Some((min, next))
            } else {
                Some((min, max))
            },
            None => Some((next, next)),
        })
    });
    max_mins
        .filter_map(|maybe_max_min| maybe_max_min.map(|(min, max)| max - min))
        .sum()
}

fn string_to_matrix(s: &str) -> Vec<Vec<isize>> {
    s.split("\n")
        .map(|v| {
            v.split("\t")
                .into_iter()
                .filter_map(|s_i| s_i.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use day_2::*;

    #[test]
    fn matrix_checksum_test() {
        let checksum = matrix_checksum(&vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]);
        assert_eq!(checksum, 18);
    }

    #[test]
    fn string_to_matrix_test() {
        let parsed = string_to_matrix(
            "5\t1\t9\t5
7\t5\t3
2\t4\t6\t8",
        );
        assert_eq!(
            parsed,
            vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]
        )
    }

}
