pub mod iterative;

pub trait Solution {
    fn is_array_special(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1] as &[_], true), (&[2, 1, 4], true), (&[4, 3, 1, 6], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_array_special(nums.to_vec()), expected);
        }
    }
}
