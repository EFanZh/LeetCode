pub mod iterative;

pub trait Solution {
    fn count_special_integers(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 8, 1, 5, 1, 5, 8, 5] as &[_], 2),
            (&[8, 8, 8, 8], 1),
            (&[8, 6, 6, 8, 8], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_special_integers(nums.to_vec()), expected);
        }
    }
}
