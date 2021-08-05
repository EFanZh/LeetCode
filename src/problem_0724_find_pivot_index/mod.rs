pub mod iterative;

pub trait Solution {
    fn pivot_index(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 7, 3, 6, 5, 6] as &[_], 3), (&[1, 2, 3], -1), (&[2, 1, -1], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::pivot_index(nums.to_vec()), expected);
        }
    }
}
