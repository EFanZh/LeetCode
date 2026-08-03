pub mod iterative;

pub trait Solution {
    fn count_beautiful_pairs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 1, 4] as &[_], 5),
            (&[11, 21, 12], 2),
            (&[84, 91, 18, 59, 27, 9, 81, 33, 17, 58], 37),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_beautiful_pairs(nums.to_vec()), expected);
        }
    }
}
