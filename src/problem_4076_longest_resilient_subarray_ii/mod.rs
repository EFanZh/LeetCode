pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 4, 6, 3] as &[_], 2), 3),
            ((&[1, 4, 7, 1], 3), 4),
            ((&[5, 5, 4, 8], 4), 2),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::resilient_subarray(nums.to_vec(), k), expected);
        }
    }
}
