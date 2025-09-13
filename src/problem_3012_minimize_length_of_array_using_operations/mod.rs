pub mod greedy;

pub trait Solution {
    fn minimum_array_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4, 3, 1] as &[_], 1), (&[5, 5, 5, 10, 5], 2), (&[2, 3, 4], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_array_length(nums.to_vec()), expected);
        }
    }
}
