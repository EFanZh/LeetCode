pub mod iterative;

pub trait Solution {
    fn count_hill_valley(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 4, 1, 1, 6, 5] as &[_], 3),
            (&[6, 6, 5, 5, 4, 1], 0),
            (&[1, 1, 1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_hill_valley(nums.to_vec()), expected);
        }
    }
}
