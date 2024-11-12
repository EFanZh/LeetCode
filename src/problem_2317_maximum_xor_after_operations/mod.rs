pub mod mathematical;

pub trait Solution {
    fn maximum_xor(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 2, 4, 6] as &[_], 7), (&[1, 2, 3, 9, 2], 11)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_xor(nums.to_vec()), expected);
        }
    }
}
