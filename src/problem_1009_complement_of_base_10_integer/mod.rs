pub mod bit_manipulation;
pub mod bit_manipulation_2;
pub mod bit_manipulation_3;

pub trait Solution {
    fn bitwise_complement(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A035327.

        let test_cases = [
            (0, 1),
            (1, 0),
            (2, 1),
            (3, 0),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 0),
            (8, 7),
            (9, 6),
            (10, 5),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::bitwise_complement(n), expected);
        }
    }
}
