pub mod count_interval_intersections;

pub trait Solution {
    fn best_rotation(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1, 4, 0] as &[_], 3), (&[1, 3, 0, 2, 4], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::best_rotation(nums.to_vec()), expected);
        }
    }
}
