pub mod iterative;

pub trait Solution {
    fn find_k_or(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[7, 12, 9, 8, 9, 15] as &[_], 4), 9),
            ((&[2, 12, 1, 11, 4, 5], 6), 0),
            ((&[10, 8, 5, 9, 11, 6, 8], 1), 15),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::find_k_or(nums.to_vec(), k), expected);
        }
    }
}
