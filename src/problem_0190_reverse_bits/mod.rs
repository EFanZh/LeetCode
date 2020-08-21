pub mod cheating;
pub mod divide_and_conquer;
pub mod iterative;

pub trait Solution {
    fn reverse_bits(n: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(43_261_596, 964_176_192), (4_294_967_293, 3_221_225_471)];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::reverse_bits(n), expected);
        }
    }
}
