pub mod mathematical;
pub mod recursive;

pub trait Solution {
    fn last_remaining(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 4),
            (7, 4),
            (8, 6),
            (9, 6),
            (10, 8),
            (11, 8),
            (12, 6),
            (13, 6),
            (14, 8),
            (15, 8),
            (16, 6),
            (17, 6),
            (18, 8),
            (19, 8),
            (20, 6),
            (21, 6),
            (22, 8),
            (23, 8),
            (24, 14),
            (25, 14),
            (26, 16),
            (27, 16),
            (28, 14),
            (29, 14),
            (30, 16),
            (31, 16),
            (32, 22),
            (33, 22),
            (34, 24),
            (35, 24),
            (36, 22),
            (37, 22),
            (38, 24),
            (39, 24),
            (40, 30),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::last_remaining(n), expected);
        }
    }
}
