pub mod greedy;

pub trait Solution {
    fn maximum_or(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[12, 9] as &[_], 1), 30), ((&[8, 1, 2], 2), 35)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_or(nums.to_vec(), k), expected);
        }
    }
}
