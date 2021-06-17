pub mod binary_search;

pub trait Solution {
    fn find_min(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 4, 5, 1, 2] as &[_], 1), (&[4, 5, 6, 7, 0, 1, 2], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_min(nums.to_vec()), expected);
        }
    }
}
