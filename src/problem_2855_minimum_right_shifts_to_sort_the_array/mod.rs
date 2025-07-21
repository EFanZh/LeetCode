pub mod greedy;

pub trait Solution {
    fn minimum_right_shifts(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 4, 5, 1, 2] as &[_], 2), (&[1, 3, 5], 0), (&[2, 1, 4], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_right_shifts(nums.to_vec()), expected);
        }
    }
}
