pub mod greedy;

pub trait Solution {
    fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], &[[1, 3], [0, 1]] as &[_]), 19),
            ((&[1, 2, 3, 4, 5, 6], &[[0, 1]]), 11),
            ((&[1, 2, 3, 4, 5, 10], &[[0, 2], [1, 3], [1, 1]]), 47),
            ((&[1, 8, 5, 5, 2], &[[4, 4], [3, 4], [4, 4], [2, 4], [0, 0]]), 49),
        ];

        for ((nums, requests), expected) in test_cases {
            assert_eq!(
                S::max_sum_range_query(nums.to_vec(), requests.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
