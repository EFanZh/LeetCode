pub mod iterative;

pub trait Solution {
    fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5, 7, 8, 9, 2, 3, 4, 3, 1] as &[_], 3), true),
            ((&[1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5), false),
            ((&[-3, -19, -8, -16], 2), false),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::has_increasing_subarrays(nums.to_vec(), k), expected);
        }
    }
}
