pub mod backtracking;

pub trait Solution {
    fn num_squareful_perms(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 17, 8] as &[_], 2),
            (&[2, 2, 2], 1),
            (&[5, 11, 5, 4, 5], 2),
            (&[1, 1, 8, 1, 8], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::num_squareful_perms(nums.to_vec()), expected);
        }
    }
}
