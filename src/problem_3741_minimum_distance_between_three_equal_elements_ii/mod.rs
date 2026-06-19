pub mod iterative;

pub trait Solution {
    fn minimum_distance(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 1, 1, 3] as &[_], 6), (&[1, 1, 2, 3, 2, 1, 2], 8), (&[1], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_distance(nums.to_vec()), expected);
        }
    }
}
