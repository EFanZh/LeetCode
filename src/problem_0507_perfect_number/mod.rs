pub mod cheating;
pub mod iterative;

pub trait Solution {
    fn check_perfect_number(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (28, true),
            (6, true),
            (496, true),
            (8128, true),
            (2, false),
            (33_550_336, true),
            (121, false),
        ];
        for (num, expected) in test_cases {
            assert_eq!(S::check_perfect_number(num), expected);
        }
    }
}
