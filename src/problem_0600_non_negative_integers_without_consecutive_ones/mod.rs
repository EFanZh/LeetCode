pub mod mathematical;

pub trait Solution {
    fn find_integers(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 2),
            (2, 3),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 5),
            (7, 5),
            (8, 6),
            (9, 7),
            (10, 8),
            (0x0001_2345, 3_338),
            (0x0012_3456, 22_879),
            (0x0123_4567, 156_815),
            (0x1234_5678, 1_074_826),
            (0x7654_3210, 3_524_578),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_integers(n), expected);
        }
    }
}
