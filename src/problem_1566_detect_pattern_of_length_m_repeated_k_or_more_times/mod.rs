pub mod iterative;

pub trait Solution {
    fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 4, 4, 4, 4] as &[_], 1, 3), true),
            ((&[1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true),
            ((&[1, 2, 1, 2, 1, 3], 2, 3), false),
        ];

        for ((arr, m, k), expected) in test_cases {
            assert_eq!(S::contains_pattern(arr.to_vec(), m, k), expected);
        }
    }
}
