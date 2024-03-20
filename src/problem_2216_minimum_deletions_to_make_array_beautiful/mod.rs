pub mod greedy;

pub trait Solution {
    fn min_deletion(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2, 3, 5] as &[_], 1), (&[1, 1, 2, 2, 3, 3], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_deletion(nums.to_vec()), expected);
        }
    }
}
