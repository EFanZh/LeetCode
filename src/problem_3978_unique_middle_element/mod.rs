pub mod iterative;

pub trait Solution {
    fn is_middle_element_unique(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], true), (&[1, 2, 2], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_middle_element_unique(nums.to_vec()), expected);
        }
    }
}
