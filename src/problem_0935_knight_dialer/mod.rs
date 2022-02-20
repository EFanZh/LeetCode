pub mod dynamic_programming;
pub mod matrix_multiplication;

pub trait Solution {
    fn knight_dialer(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 10),
            (2, 20),
            (3, 46),
            (4, 104),
            (5, 240),
            (6, 544),
            (7, 1256),
            (8, 2848),
            (9, 6576),
            (10, 14912),
            (11, 34432),
            (12, 78080),
            (13, 180_288),
            (14, 408_832),
            (15, 944_000),
            (16, 2_140_672),
            (3131, 136_006_598),
            (5000, 406_880_451),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::knight_dialer(n), expected);
        }
    }
}
