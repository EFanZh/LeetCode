pub mod reduce_to_three_sum;

pub trait Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![
            (
                (vec![1, 0, -1, 0, -2, 2], 0),
                vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]],
            ),
            ((vec![], 0), vec![]),
            ((vec![0, 0, 0, 0], 0), vec![vec![0, 0, 0, 0]]),
            (
                (vec![0, 1, 5, 0, 1, 5, 5, -4], 11),
                vec![vec![-4, 5, 5, 5], vec![0, 1, 5, 5]],
            ),
            ((vec![-3, -1, 0, 2, 4, 5], 0), vec![vec![-3, -1, 0, 4]]),
            ((vec![2, 1, 0, -1], 2), vec![vec![-1, 0, 1, 2]]),
            (
                (vec![-3, -2, -1, 0, 0, 1, 2, 3], 0),
                vec![
                    vec![-3, -2, 2, 3],
                    vec![-3, -1, 1, 3],
                    vec![-3, 0, 0, 3],
                    vec![-3, 0, 1, 2],
                    vec![-2, -1, 0, 3],
                    vec![-2, -1, 1, 2],
                    vec![-2, 0, 0, 2],
                    vec![-1, 0, 0, 1],
                ],
            ),
        ];

        fn sorted<T: Ord>(v: Vec<T>) -> Vec<T> {
            let mut v = v;

            v.sort_unstable();

            v
        }

        for ((nums, target), expected) in test_cases {
            assert_eq!(sorted(S::four_sum(nums, target)), sorted(expected));
        }
    }
}
