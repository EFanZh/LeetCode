pub mod prefix_sums;

pub trait Solution {
    fn subarray_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1] as &[_], 11), (&[3, 1, 1, 2], 13)];

        for (nums, expected) in test_cases {
            assert_eq!(S::subarray_sum(nums.to_vec()), expected);
        }
    }
}
