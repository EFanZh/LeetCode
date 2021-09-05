pub mod bfs;
pub mod bfs_2;
pub mod bfs_3;

pub trait Solution {
    fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["01000001", "01000001", "00000001", "00000000"] as &[_], 10),
            (&["111", "101", "111"], 4),
            (&["111000000", "101011111", "111000000"], 13),
            (&["11111", "11111", "11111", "11111", "11111"], 0),
        ];

        for (is_infected, expected) in test_cases {
            assert_eq!(
                S::contain_virus(
                    is_infected
                        .iter()
                        .map(|s| s.bytes().map(|c| i32::from(c - b'0')).collect())
                        .collect()
                ),
                expected
            );
        }
    }
}
