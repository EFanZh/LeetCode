pub mod sliding_window;

pub trait Solution {
    fn maximum_beauty(flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 1, 1] as &[_], 7, 6, 12, 1), 14),
            ((&[2, 4, 5, 3], 10, 5, 2, 6), 30),
            ((&[18, 16, 10, 10, 5], 10, 3, 15, 4), 75),
            ((&[8, 2], 24, 18, 6, 3), 54),
            ((&[20, 1, 15, 17, 10, 2, 4, 16, 15, 11], 2, 20, 10, 2), 14),
        ];

        for ((flowers, new_flowers, target, full, partial), expected) in test_cases {
            assert_eq!(
                S::maximum_beauty(flowers.to_vec(), new_flowers, target, full, partial),
                expected,
            );
        }
    }
}
