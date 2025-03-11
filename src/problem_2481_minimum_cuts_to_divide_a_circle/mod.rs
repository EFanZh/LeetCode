pub mod bit_manipulation;

pub trait Solution {
    fn number_of_cuts(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 0),
            (2, 1),
            (3, 3),
            (4, 2),
            (5, 5),
            (6, 3),
            (7, 7),
            (8, 4),
            (9, 9),
            (10, 5),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::number_of_cuts(n), expected);
        }
    }
}
