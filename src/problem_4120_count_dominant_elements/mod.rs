pub mod iterative;

pub trait Solution {
    fn count_dominant_elements(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 10, 2] as &[_], 1), (&[6, 4, 2], 2), (&[7], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_dominant_elements(nums.to_vec()), expected);
        }
    }
}
