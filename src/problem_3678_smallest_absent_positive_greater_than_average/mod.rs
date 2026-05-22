pub mod sliding_window;

pub trait Solution {
    fn smallest_absent(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 5] as &[_], 6), (&[-1, 1, 2], 3), (&[4, -1], 2), (&[-34], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::smallest_absent(nums.to_vec()), expected);
        }
    }
}
