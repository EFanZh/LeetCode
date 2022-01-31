pub mod iterative;

pub trait Solution {
    fn partition_disjoint(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 0, 3, 8, 6] as &[_], 3), (&[1, 1, 1, 0, 6, 12], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::partition_disjoint(nums.to_vec()), expected);
        }
    }
}
