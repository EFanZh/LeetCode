pub mod greedy;

pub trait Solution {
    fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 1] as &[_], 3, &[[0, 1], [0, 2]] as &[_]), 6),
            ((&[2, 3], 7, &[[0, 1]]), 9),
            ((&[7, 7, 7, 7, 7, 7], 3, &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]]), 42),
            ((&[24, 78, 1, 97, 44], 6, &[[0, 2], [1, 2], [4, 2], [3, 4]]), 260),
            (
                (
                    &[0, 92, 56, 3, 34, 23, 56],
                    7,
                    &[[2, 6], [4, 1], [5, 0], [1, 0], [3, 1], [6, 3]],
                ),
                288,
            ),
        ];

        for ((nums, k, edges), expected) in test_cases {
            assert_eq!(
                S::maximum_value_sum(nums.to_vec(), k, edges.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
