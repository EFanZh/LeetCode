pub mod iterative;

pub trait Solution {
    fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 5, 3, 9, 8] as &[_], 2), &[1, 3, 5, 8, 9] as &[_]),
            ((&[1, 7, 6, 18, 2, 1], 3), &[1, 6, 7, 18, 1, 2]),
            ((&[1, 7, 28, 19, 10], 3), &[1, 7, 28, 19, 10]),
        ];

        for ((nums, limit), expected) in test_cases {
            assert_eq!(S::lexicographically_smallest_array(nums.to_vec(), limit), expected);
        }
    }
}
