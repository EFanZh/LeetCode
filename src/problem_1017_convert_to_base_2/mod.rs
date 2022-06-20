pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn base_neg2(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, "0"),
            (2, "110"),
            (3, "111"),
            (4, "100"),
            (1_000_000_000, "1001100111011111101111000000000"),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::base_neg2(n), expected);
        }
    }
}
