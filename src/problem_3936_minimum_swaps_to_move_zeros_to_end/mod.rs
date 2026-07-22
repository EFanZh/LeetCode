pub mod two_pointers;

pub trait Solution {
    fn minimum_swaps(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 0, 3, 12] as &[_], 2), (&[0, 1, 0, 2], 1), (&[1, 2, 0], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_swaps(nums.to_vec()), expected);
        }
    }
}
