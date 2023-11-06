pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 5, 3], [1, 8, 4], [1, 7, 5]] as &[_], [2, 7, 5]), true),
            ((&[[3, 4, 5], [4, 5, 6]], [3, 2, 5]), false),
            ((&[[2, 5, 3], [2, 3, 4], [1, 2, 5], [5, 2, 3]], [5, 5, 5]), true),
            ((&[[1, 2, 5], [1, 7, 4]], [2, 7, 5]), false),
            ((&[[1, 5, 7], [3, 10, 1]], [3, 5, 7]), false),
            ((&[[3, 1, 7], [10, 5, 7]], [3, 5, 7]), false),
            ((&[[1, 3, 1]], [1, 3, 2]), false),
            ((&[[1, 3, 4], [2, 5, 8]], [2, 5, 8]), true),
            (
                (
                    &[
                        [15, 10, 2],
                        [2, 12, 8],
                        [14, 2, 13],
                        [9, 13, 3],
                        [3, 11, 8],
                        [9, 3, 5],
                        [9, 14, 10],
                        [7, 4, 15],
                        [9, 13, 12],
                        [12, 4, 12],
                    ],
                    [9, 14, 15],
                ),
                true,
            ),
        ];

        for ((triplets, target), expected) in test_cases {
            assert_eq!(
                S::merge_triplets(triplets.iter().copied().map(Vec::from).collect(), target.into()),
                expected,
            );
        }
    }
}
