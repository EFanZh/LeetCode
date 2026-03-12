pub mod sliding_window;

pub trait Solution {
    fn max_adjacent_distance(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 4] as &[_], 3), (&[-5, -10, -5], 5)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_adjacent_distance(nums.to_vec()), expected);
        }
    }
}
