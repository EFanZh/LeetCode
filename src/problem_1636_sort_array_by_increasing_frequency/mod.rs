pub mod iterative;

pub trait Solution {
    fn frequency_sort(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2, 2, 2, 3] as &[_], &[3, 1, 1, 2, 2, 2] as &[_]),
            (&[2, 3, 1, 3, 2], &[1, 3, 3, 2, 2]),
            (&[-1, 1, -6, 4, 5, -6, 1, 4, 1], &[5, -1, 4, 4, -6, -6, 1, 1, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::frequency_sort(nums.to_vec()), expected);
        }
    }
}
