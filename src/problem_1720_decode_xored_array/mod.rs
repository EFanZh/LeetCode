pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], 1), &[1, 0, 2, 1] as &[_]),
            ((&[6, 2, 7, 3], 4), &[4, 2, 0, 7, 4]),
        ];

        for ((encoded, first), expected) in test_cases {
            assert_eq!(S::decode(encoded.to_vec(), first), expected);
        }
    }
}
