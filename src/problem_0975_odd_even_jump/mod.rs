pub mod dynamic_programming;

pub trait Solution {
    fn odd_even_jumps(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[10, 13, 12, 14, 15] as &[_], 2),
            (&[2, 3, 1, 1, 4], 3),
            (&[5, 1, 3, 4, 2], 3),
            (&[5, 4, 3, 2, 1], 1),
            (&[1, 2, 3, 2, 1, 4, 4, 5], 6),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::odd_even_jumps(arr.to_vec()), expected);
        }
    }
}
