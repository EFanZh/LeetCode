pub mod iterative;

pub trait Solution {
    fn min_element(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[10, 12, 13, 14] as &[_], 1),
            (&[1, 2, 3, 4], 1),
            (&[999, 19, 199], 10),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_element(nums.to_vec()), expected);
        }
    }
}
