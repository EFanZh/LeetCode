pub mod hash_set;

pub trait Solution {
    fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[3, 1, 6, 8, 4] as &[_], 24), true), ((&[2, 5, 3, 7], 15), false)];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::check_equal_partitions(nums.to_vec(), target), expected);
        }
    }
}
