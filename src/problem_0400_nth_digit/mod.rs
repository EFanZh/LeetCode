pub mod binary_search;
pub mod iterative;

pub trait Solution {
    fn find_nth_digit(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 1),
            (5, 5),
            (9, 9),
            (10, 1),
            (11, 0),
            (1_000_000_000, 1),
            (2_147_483_647, 2),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::find_nth_digit(n), expected);
        }
    }
}
