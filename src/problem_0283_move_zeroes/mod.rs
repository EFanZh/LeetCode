pub mod iterative;

pub trait Solution {
    fn move_zeroes(nums: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 0, 3, 12] as &[_], &[1, 3, 12, 0, 0] as &[_])];

        for (nums, expected) in test_cases.iter().copied() {
            let mut nums = nums.to_vec();

            S::move_zeroes(&mut nums);

            assert_eq!(nums, expected);
        }
    }
}
