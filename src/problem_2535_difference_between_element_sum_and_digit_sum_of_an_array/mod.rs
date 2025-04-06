pub mod iterative;

pub trait Solution {
    fn difference_of_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 15, 6, 3] as &[_], 9), (&[1, 2, 3, 4], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::difference_of_sum(nums.to_vec()), expected);
        }
    }
}
