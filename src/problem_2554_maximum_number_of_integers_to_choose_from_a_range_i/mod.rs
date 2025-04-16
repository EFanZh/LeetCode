pub mod greedy;

pub trait Solution {
    fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 6, 5] as &[_], 5, 6), 2),
            ((&[1, 2, 3, 4, 5, 6, 7], 8, 1), 0),
            ((&[11], 7, 50), 7),
            (
                (
                    &[
                        87, 97, 20, 17, 13, 36, 89, 18, 39, 33, 30, 32, 57, 71, 4, 90, 17, 3, 19, 45, 33, 20, 7, 8, 75,
                        99, 74, 33, 12, 76, 67, 88, 59, 15, 28, 20, 20, 8, 8, 32, 75,
                    ],
                    5080,
                    14,
                ),
                4,
            ),
        ];

        for ((banned, n, max_sum), expected) in test_cases {
            assert_eq!(S::max_count(banned.to_vec(), n, max_sum), expected);
        }
    }
}
