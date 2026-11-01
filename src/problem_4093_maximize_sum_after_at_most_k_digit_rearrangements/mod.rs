pub mod greedy;

pub trait Solution {
    fn maximize_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[123, 45] as &[_], 1), 366), ((&[102, 90, 14], 2), 341)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximize_sum(nums.to_vec(), k), expected);
        }
    }
}
