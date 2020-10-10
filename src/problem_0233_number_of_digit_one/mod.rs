pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn count_digit_one(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (-1, 0),
            (10, 2),
            (13, 6),
            (20, 12),
            (101, 23),
            (1_410_065_408, 1_737_167_499),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_digit_one(n), expected);
        }
    }
}
