pub mod dynamic_programming;

pub trait Solution {
    fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 5, 6] as &[_], &[2, -6, 4, -5, -3, 2, -7] as &[_]), 26),
            ((&[-1, 4, 5, -2], &[-5, -1, -3, -2, -4]), -1),
        ];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::max_score(a.to_vec(), b.to_vec()), expected);
        }
    }
}
