pub mod iterative;

pub trait Solution {
    fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 2, 1] as &[_], &[3, 1, 2] as &[_]),
            (&[1, 1, 5], &[1, 1, 5]),
            (&[1, 9, 4, 6, 7], &[1, 7, 4, 6, 9]),
            (&[3, 1, 1, 3], &[1, 3, 1, 3]),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::prev_perm_opt1(arr.to_vec()), expected);
        }
    }
}
