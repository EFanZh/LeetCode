pub mod greedy;

pub trait Solution {
    fn max_total_value(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3, 2] as &[_], 2), 4), ((&[4, 2, 5, 1], 3), 12)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_total_value(nums.to_vec(), k), expected);
        }
    }
}
