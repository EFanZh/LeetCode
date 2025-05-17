pub mod prefix_sums_and_binary_search;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 6, 8] as &[_], &[1, 5] as &[_]), &[14_i64, 10] as &[_]),
            ((&[2, 9, 6, 3], &[10]), &[20]),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), queries.to_vec()), expected);
        }
    }
}
