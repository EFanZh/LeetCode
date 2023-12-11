pub mod sliding_window;

pub trait Solution {
    fn min_moves(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 0, 0, 1, 0, 1] as &[_], 2), 1),
            ((&[1, 0, 0, 0, 0, 0, 1, 1], 3), 5),
            ((&[1, 1, 0, 1], 2), 0),
            ((&[0, 1, 1, 0, 0, 1, 0, 0, 0], 3), 2),
            ((&[1, 1], 1), 0),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_moves(nums.to_vec(), k), expected);
        }
    }
}
