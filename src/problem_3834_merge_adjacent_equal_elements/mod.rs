pub mod stack;

pub trait Solution {
    fn merge_adjacent(nums: Vec<i32>) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 1, 2] as &[_], &[3_i64, 4] as &[_]),
            (&[2, 2, 4], &[8]),
            (&[3, 7, 5], &[3, 7, 5]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::merge_adjacent(nums.to_vec()), expected);
        }
    }
}
