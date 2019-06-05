pub mod sort_then_two_sum;
pub mod sort_then_two_sum_short;

pub trait Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![
            (vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, 0, 1], vec![-1, -1, 2]]),
            (vec![], vec![]),
            (
                vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
                vec![
                    vec![-4, -2, 6],
                    vec![-4, 0, 4],
                    vec![-4, 1, 3],
                    vec![-4, 2, 2],
                    vec![-2, -2, 4],
                    vec![-2, 0, 2],
                ],
            ),
            (vec![0; 3000], vec![vec![0, 0, 0]]),
            (vec![-1, 0, 1], vec![vec![-1, 0, 1]]),
            (vec![1, 1, 1], vec![]),
        ];

        fn sorted<T: Ord>(v: Vec<T>) -> Vec<T> {
            let mut v = v;

            v.sort_unstable();

            v
        }

        for (nums, expected) in test_cases {
            assert_eq!(sorted(S::three_sum(nums)), sorted(expected));
        }
    }
}
