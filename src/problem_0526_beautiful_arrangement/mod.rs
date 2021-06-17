pub mod backtracking;

pub trait Solution {
    fn count_arrangement(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A320843.
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 8),
            (5, 10),
            (6, 36),
            (7, 41),
            (8, 132),
            (9, 250),
            (10, 700),
            (11, 750),
            (12, 4010),
            (13, 4237),
            (14, 10680),
            (15, 24679),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::count_arrangement(n), expected);
        }
    }
}
