pub mod cheating;
pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn hamming_weight(n: u32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(11, 3), (128, 1), (4_294_967_294, 31)];

        for (n, expected) in test_cases {
            assert_eq!(S::hamming_weight(n), expected);
        }
    }
}
