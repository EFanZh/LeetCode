pub mod iterative;

pub trait Solution {
    fn is_good_array(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[12, 5, 7, 23] as &[_], true), (&[29, 6, 10], true), (&[3, 6], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_good_array(nums.to_vec()), expected);
        }
    }
}
