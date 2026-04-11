pub mod greedy;

pub trait Solution {
    fn maximum_possible_size(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 2, 5, 3, 5] as &[_], 3), (&[1, 2, 3], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_possible_size(nums.to_vec()), expected);
        }
    }
}
