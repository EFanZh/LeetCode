pub mod dynamic_programming;

pub trait Solution {
    fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[2, 5, 4], [1, 5, 1]] as &[_]), 7),
            (
                (
                    20,
                    &[
                        [1, 6, 1],
                        [3, 10, 2],
                        [10, 12, 3],
                        [11, 12, 2],
                        [12, 15, 2],
                        [13, 18, 1],
                    ],
                ),
                20,
            ),
        ];

        for ((n, rides), expected) in test_cases {
            assert_eq!(S::max_taxi_earnings(n, rides.iter().map(Vec::from).collect()), expected);
        }
    }
}
