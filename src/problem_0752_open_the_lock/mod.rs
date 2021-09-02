pub mod bfs;
pub mod bfs_2;
pub mod bidirectional_bfs;

pub trait Solution {
    fn open_lock(deadends: Vec<String>, target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["0201", "0101", "0102", "1212", "2002"] as &[_], "0202"), 6),
            ((&["8888"], "0009"), 1),
            (
                (
                    &["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
                    "8888",
                ),
                -1,
            ),
            ((&["0000"], "8888"), -1),
            ((&["1243"], "0000"), 0),
            ((&["2110", "0202", "1222", "2221", "1010"], "2010"), 3),
            (
                (
                    &["1000", "0100", "0010", "0001", "9000", "0900", "0090", "0009"],
                    "0202",
                ),
                -1,
            ),
        ];

        for ((deadends, target), expected) in test_cases {
            assert_eq!(
                S::open_lock(
                    deadends.iter().copied().map(str::to_string).collect(),
                    target.to_string()
                ),
                expected
            );
        }
    }
}
