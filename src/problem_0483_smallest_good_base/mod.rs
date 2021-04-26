pub mod iterative;

pub trait Solution {
    fn smallest_good_base(n: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (3_u64, 2),
            (4, 3),
            (5, 4),
            (6, 5),
            (7, 2),
            (8, 7),
            (9, 8),
            (10, 9),
            (11, 10),
            (12, 11),
            (13, 3),
            (14, 13),
            (15, 2),
            (16, 15),
            (4681, 8),
            (1_000_000_000_000_000_000, 999_999_999_999_999_999),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::smallest_good_base(n.to_string()).parse::<u64>().unwrap(), expected);
        }
    }
}
