pub mod memoized_dynamic_programming;

pub trait Solution {
    fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[2, 1], [3, 1], [1, 4]] as &[_], 2), 3),
            ((5, &[[2, 1], [3, 1], [4, 1], [1, 5]], 2), 4),
            ((9, &[[1, 5], [2, 5], [3, 5], [4, 6], [4, 7], [4, 8], [4, 9]], 3), 3),
            (
                (
                    12,
                    &[
                        [11, 10],
                        [6, 3],
                        [2, 5],
                        [9, 2],
                        [4, 12],
                        [8, 7],
                        [9, 5],
                        [6, 2],
                        [7, 2],
                        [7, 4],
                        [9, 3],
                        [11, 1],
                        [4, 3],
                    ],
                    3,
                ),
                4,
            ),
        ];

        for ((n, relations, k), expected) in test_cases {
            assert_eq!(
                S::min_number_of_semesters(n, relations.iter().copied().map(Vec::from).collect(), k),
                expected
            );
        }
    }
}
