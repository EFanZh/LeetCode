pub mod prefix_sum;

pub trait Solution {
    fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 3, 4, 8] as &[_], &[[0, 1], [1, 2], [2, 3], [0, 3]] as &[_]),
                &[2, 1, 4, 1] as &[_],
            ),
            (
                (&[4, 5, 2, 2, 7, 10], &[[2, 3], [0, 2], [0, 5], [3, 5]]),
                &[-1, 1, 1, 3],
            ),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(
                S::min_difference(nums.to_vec(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
