pub mod iterative;

pub trait Solution {
    fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[12, 54, 32, 22] as &[_], 2), 4), ((&[1, 34, 7], 9), 0)];

        for ((nums, digit), expected) in test_cases {
            assert_eq!(S::count_digit_occurrences(nums.to_vec(), digit), expected);
        }
    }
}
