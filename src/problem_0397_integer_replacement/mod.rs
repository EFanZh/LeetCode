pub mod dynamic_programming;
pub mod greedy;
pub mod recursive;

pub trait Solution {
    fn integer_replacement(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 4),
            (8, 3),
            (9, 4),
            (10, 4),
            (11, 5),
            (12, 4),
            (13, 5),
            (14, 5),
            (15, 5),
            (16, 4),
            (17, 5),
            (18, 5),
            (19, 6),
            (20, 5),
            (65535, 17),
            (0x5555_5555, 45),
            (0x7fff_ffff, 32),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::integer_replacement(n), expected);
        }
    }
}
