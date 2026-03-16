pub mod iterative;

pub trait Solution {
    fn count_partitions(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 10, 3, 7, 6] as &[_], 4), (&[1, 2, 2], 0), (&[2, 4, 6, 8], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_partitions(nums.to_vec()), expected);
        }
    }
}
