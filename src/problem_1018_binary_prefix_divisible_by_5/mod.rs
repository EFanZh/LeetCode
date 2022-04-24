pub mod modular_arithmetic;

pub trait Solution {
    fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 1] as &[_], &[true, false, false] as &[_]),
            (&[1, 1, 1], &[false, false, false]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::prefixes_div_by5(nums.to_vec()), expected);
        }
    }
}
