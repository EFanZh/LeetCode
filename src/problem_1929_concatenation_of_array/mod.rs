pub mod cheating;

pub trait Solution {
    fn get_concatenation(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 1] as &[_], &[1, 2, 1, 1, 2, 1] as &[_]),
            (&[1, 3, 2, 1], &[1, 3, 2, 1, 1, 3, 2, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::get_concatenation(nums.to_vec()), expected);
        }
    }
}
