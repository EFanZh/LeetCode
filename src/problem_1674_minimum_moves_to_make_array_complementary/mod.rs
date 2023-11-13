pub mod sweep_line;

pub trait Solution {
    fn min_moves(nums: Vec<i32>, limit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 4, 3] as &[_], 4), 1),
            ((&[1, 2, 2, 1], 2), 2),
            ((&[1, 2, 1, 2], 2), 0),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_moves(nums.to_vec(), k), expected);
        }
    }
}
