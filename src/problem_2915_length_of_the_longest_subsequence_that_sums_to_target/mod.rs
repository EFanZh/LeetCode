pub mod dynamic_programming;

pub trait Solution {
    fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 9), 3),
            ((&[4, 1, 3, 2, 1, 5], 7), 4),
            ((&[1, 1, 5, 4, 5], 3), -1),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::length_of_longest_subsequence(nums.to_vec(), target), expected);
        }
    }
}
