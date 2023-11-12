pub mod iterative;

pub trait Solution {
    fn is_three(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, false),
            (2, false),
            (3, false),
            (4, true),
            (5, false),
            (6, false),
            (7, false),
            (8, false),
            (9, true),
            (10, false),
            (11, false),
            (12, false),
            (13, false),
            (14, false),
            (15, false),
            (16, false),
            (17, false),
            (18, false),
            (19, false),
            (20, false),
            (21, false),
            (22, false),
            (23, false),
            (24, false),
            (25, true),
            (27, false),
            (28, false),
            (29, false),
            (30, false),
            (31, false),
            (32, false),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::is_three(n), expected);
        }
    }
}
