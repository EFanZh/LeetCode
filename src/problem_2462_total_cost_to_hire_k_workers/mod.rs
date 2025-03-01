pub mod binary_heap;

pub trait Solution {
    fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[17, 12, 10, 2, 7, 2, 11, 20, 8] as &[_], 3, 4), 11),
            ((&[1, 2, 4, 1], 3, 3), 4),
            ((&[57, 33, 26, 76, 14, 67, 24, 90, 72, 37, 30], 11, 2), 526),
            (
                (
                    &[
                        25, 65, 41, 31, 14, 20, 59, 42, 43, 57, 73, 45, 30, 77, 17, 38, 20, 11, 17, 65, 55, 85, 74, 32,
                        84,
                    ],
                    24,
                    8,
                ),
                1035,
            ),
            ((&[3, 3, 1, 1, 1], 4, 2), 6),
        ];

        for ((costs, k, candidates), expected) in test_cases {
            assert_eq!(S::total_cost(costs.to_vec(), k, candidates), expected);
        }
    }
}
