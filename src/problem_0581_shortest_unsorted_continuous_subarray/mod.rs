pub mod iterative;

pub trait Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 6, 4, 8, 10, 9, 15] as &[_], 5), (&[1, 2, 3, 4], 0), (&[1], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_unsorted_subarray(nums.to_vec()), expected);
        }
    }
}
