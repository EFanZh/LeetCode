pub mod iterative;

pub trait Solution {
    fn capture_forts(forts: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 0, 0, -1, 0, 0, 0, 0, 1] as &[_], 4),
            (&[0, 0, 1, -1], 0),
            (&[1, 0, 0, -1, 0, 0, -1, 0, 0, 1], 2),
        ];

        for (forts, expected) in test_cases {
            assert_eq!(S::capture_forts(forts.to_vec()), expected);
        }
    }
}
