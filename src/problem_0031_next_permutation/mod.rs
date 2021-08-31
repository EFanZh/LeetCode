pub mod find_last_inversion;

pub trait Solution {
    fn next_permutation(nums: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], &[1, 3, 2] as &[_]),
            (&[3, 2, 1], &[1, 2, 3]),
            (&[1, 1, 5], &[1, 5, 1]),
            (&[1], &[1]),
            (&[1, 5, 1], &[5, 1, 1]),
            (&[1, 3, 2], &[2, 1, 3]),
        ];

        for (nums, expected) in test_cases {
            let mut nums = nums.to_vec();

            S::next_permutation(&mut nums);

            assert_eq!(nums.as_slice(), expected);
        }
    }
}
