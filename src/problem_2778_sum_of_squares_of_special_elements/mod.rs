pub mod iterative;

pub trait Solution {
    fn sum_of_squares(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], 21), (&[2, 7, 1, 19, 18, 3], 63)];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_of_squares(nums.to_vec()), expected);
        }
    }
}
