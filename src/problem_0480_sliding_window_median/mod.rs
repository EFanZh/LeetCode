pub mod two_binary_heaps;
pub mod two_sets;

pub trait Solution {
    fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 3, -1, -3, 5, 3, 6, 7] as &[_], 3),
                &[1.0, -1.0, -1.0, 3.0, 5.0, 6.0] as &[_],
            ),
            ((&[1, 2, 3, 4, 2, 3, 1, 4, 2], 3), &[2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0]),
            ((&[1], 1), &[1.0]),
            (
                (&[4, 1, 7, 1, 8, 7, 8, 7, 7, 4], 4),
                &[2.5, 4.0, 7.0, 7.5, 7.5, 7.0, 7.0],
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::median_sliding_window(nums.to_vec(), k), expected);
        }
    }
}
