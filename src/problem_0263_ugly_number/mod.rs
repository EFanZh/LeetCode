pub mod brute_force;

pub trait Solution {
    fn is_ugly(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (-2, false),
            (-1, false),
            (0, false),
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (7, false),
            (8, true),
            (9, true),
            (10, true),
            (11, false),
            (12, true),
            (13, false),
            (14, false),
        ];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_ugly(num), expected);
        }
    }
}
