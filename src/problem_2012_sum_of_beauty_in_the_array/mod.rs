pub mod iterative;

pub trait Solution {
    fn sum_of_beauties(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 2), (&[2, 4, 6, 4], 1), (&[3, 2, 1], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_of_beauties(nums.to_vec()), expected);
        }
    }
}
