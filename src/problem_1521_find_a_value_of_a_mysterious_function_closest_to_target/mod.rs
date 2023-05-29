pub mod sliding_window;
pub mod sliding_window_masked;

pub trait Solution {
    fn closest_to_target(arr: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        const MASK: i32 = (-1 << 11) >> 1;

        let test_cases = [
            ((&[9, 12, 3, 7, 15] as &[_], 5), 2),
            ((&[1_000_000, 1_000_000, 1_000_000], 1), 999_999),
            ((&[1, 2, 4, 8, 16], 0), 0),
            ((&[1, 2, 4, 8, 16], 0), 0),
            ((&[6, 5, 1], 1), 0),
            ((&[5, 89, 79, 44, 45, 79], 965), 876),
            (
                (
                    &[5 | MASK, 89 | MASK, 79 | MASK, 44 | MASK, 45 | MASK, 79 | MASK],
                    965 | MASK,
                ),
                876,
            ),
            ((&[10, 7, 10, 5, 10, 1, 10, 7, 7, 1], 127), 117),
        ];

        for ((arr, target), expected) in test_cases {
            assert_eq!(S::closest_to_target(arr.to_vec(), target), expected);
        }
    }
}
