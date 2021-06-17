pub mod dynamic_programming;

pub trait Solution {
    fn can_win_nim(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, true),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, true),
            (7, true),
            (8, false),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::can_win_nim(n), expected);
        }
    }
}
