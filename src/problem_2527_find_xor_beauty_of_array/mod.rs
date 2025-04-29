pub mod mathematical;

pub trait Solution {
    fn xor_beauty(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4] as &[_], 5), (&[15, 45, 20, 2, 34, 35, 5, 44, 32, 30], 34)];

        for (nums, expected) in test_cases {
            assert_eq!(S::xor_beauty(nums.to_vec()), expected);
        }
    }
}
