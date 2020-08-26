pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn range_bitwise_and(m: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 7), 4), ((0, 1), 0), ((2, 2), 2)];

        for ((m, n), expected) in test_cases.iter().copied() {
            assert_eq!(S::range_bitwise_and(m, n), expected);
        }
    }
}
