pub mod dynamic_programming;

pub trait Solution {
    fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[0, 0, 1], [0, 2, 2], [1, 3, 2]] as &[_]), 3),
            ((5, &[[0, 0, 1], [0, 2, 10], [1, 3, 2]]), 10),
            (
                (
                    10,
                    &[
                        [1, 6, 1],
                        [0, 1, 10],
                        [3, 6, 2],
                        [0, 5, 10],
                        [0, 0, 3],
                        [0, 0, 4],
                        [1, 1, 4],
                        [0, 6, 7],
                        [4, 4, 1],
                    ],
                ),
                12,
            ),
        ];

        for ((n, offers), expected) in test_cases {
            assert_eq!(
                S::maximize_the_profit(n, offers.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
