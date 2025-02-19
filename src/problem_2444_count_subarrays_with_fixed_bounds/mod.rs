pub mod greedy;

pub trait Solution {
    fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3, 5, 2, 7, 5] as &[_], 1, 5), 2), ((&[1, 1, 1, 1], 1, 1), 10)];

        for ((nums, min_k, max_k), expected) in test_cases {
            assert_eq!(S::count_subarrays(nums.to_vec(), min_k, max_k), expected);
        }
    }
}
