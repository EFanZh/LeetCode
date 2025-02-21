pub mod binary_search;

pub trait Solution {
    fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 2] as &[_], &[2, 3, 1, 14] as &[_]), 8),
            ((&[2, 2, 2, 2, 2], &[4, 2, 8, 1, 3]), 0),
        ];

        for ((nums, cost), expected) in test_cases {
            assert_eq!(S::min_cost(nums.to_vec(), cost.to_vec()), expected,);
        }
    }
}
