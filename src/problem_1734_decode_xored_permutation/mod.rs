pub mod iterative;

pub trait Solution {
    fn decode(encoded: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 1] as &[_], &[1, 2, 3] as &[_]), (&[6, 5, 4, 6], &[2, 4, 1, 5, 3])];

        for (encoded, expected) in test_cases {
            assert_eq!(S::decode(encoded.to_vec()), expected);
        }
    }
}
