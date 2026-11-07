pub mod iterative;

pub trait Solution {
    fn min_merged_integer(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[23, 55, 42, 21] as &[_], 2355),
            (&[34, 7, 11, 11], 347),
            (&[9, 20], 920),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_merged_integer(nums.to_vec()), expected);
        }
    }
}
