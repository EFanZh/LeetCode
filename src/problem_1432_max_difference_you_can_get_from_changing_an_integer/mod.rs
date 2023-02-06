pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn max_diff(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (555, 888),
            (9, 8),
            (123_456, 820_000),
            (111, 888),
            (9, 8),
            (99_796, 88_280),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::max_diff(num), expected);
        }
    }
}
