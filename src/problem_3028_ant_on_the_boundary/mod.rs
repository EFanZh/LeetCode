pub mod prefix_sums;

pub trait Solution {
    fn return_to_boundary_count(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, -5] as &[_], 1), (&[3, 2, -3, -4], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::return_to_boundary_count(nums.to_vec()), expected);
        }
    }
}
