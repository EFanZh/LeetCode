pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn trailing_zeroes(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (3, 0),
            (5, 1),
            (30, 7),
            (1_808_548_329, 452_137_076),
            (2_147_483_647, 536_870_902),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::trailing_zeroes(n), expected);
        }
    }
}
