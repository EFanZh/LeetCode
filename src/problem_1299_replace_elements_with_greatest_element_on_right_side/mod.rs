pub mod iterative;

pub trait Solution {
    fn replace_elements(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[17, 18, 5, 4, 6, 1] as &[_], &[18, 6, 6, 6, 1, -1] as &[_]),
            (&[400], &[-1]),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::replace_elements(arr.to_vec()), expected);
        }
    }
}
