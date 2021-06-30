pub mod iterative;

pub trait Solution {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5, 4, 7] as &[_], 3), (&[2, 2, 2, 2, 2], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_length_of_lcis(nums.to_vec()), expected);
        }
    }
}
