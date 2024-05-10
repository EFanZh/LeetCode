pub mod brute_force;

pub trait Solution {
    fn count_triples(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // See <https://oeis.org/A063468>.
        let test_cases = [
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 2),
            (10, 4),
            (11, 4),
            (12, 4),
            (13, 6),
            (14, 6),
            (15, 8),
            (16, 8),
            (17, 10),
            (18, 10),
            (19, 10),
            (20, 12),
            (21, 12),
            (22, 12),
            (23, 12),
            (24, 12),
            (25, 16),
            (26, 18),
            (27, 18),
            (28, 18),
            (29, 20),
            (30, 22),
            (31, 22),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::count_triples(n), expected);
        }
    }
}
