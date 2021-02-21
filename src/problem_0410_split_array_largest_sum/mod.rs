pub mod binary_search;

pub trait Solution {
    fn split_array(nums: Vec<i32>, m: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[7, 2, 5, 10, 8] as &[_], 2), 18),
            ((&[1, 2, 3, 4, 5], 2), 9),
            ((&[1, 4, 4], 3), 4),
        ];

        for ((nums, m), expected) in test_cases.iter().copied() {
            assert_eq!(S::split_array(nums.to_vec(), m), expected);
        }
    }
}
