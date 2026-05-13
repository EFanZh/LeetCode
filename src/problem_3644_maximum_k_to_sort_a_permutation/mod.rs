pub mod mathematical;

pub trait Solution {
    fn sort_permutation(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 3, 2, 1] as &[_], 1),
            (&[0, 1, 3, 2], 2),
            (&[3, 2, 1, 0], 0),
            (&[0, 1, 2, 3, 7, 6, 4, 5], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sort_permutation(nums.to_vec()), expected);
        }
    }
}
