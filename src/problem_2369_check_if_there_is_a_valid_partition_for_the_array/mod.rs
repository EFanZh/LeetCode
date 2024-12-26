pub mod dynamic_programming;

pub trait Solution {
    fn valid_partition(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 4, 4, 5, 6] as &[_], true), (&[1, 1, 1, 2], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::valid_partition(nums.to_vec()), expected);
        }
    }
}
