pub mod binary_search;

pub trait Solution {
    fn super_egg_drop(k: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 1), 1),
            ((1, 2), 2),
            ((1, 3), 3),
            ((1, 4), 4),
            ((1, 5), 5),
            ((1, 6), 6),
            ((1, 7), 7),
            ((1, 8), 8),
            ((2, 1), 1),
            ((2, 2), 2),
            ((2, 3), 2),
            ((2, 4), 3),
            ((2, 5), 3),
            ((2, 6), 3),
            ((2, 7), 4),
            ((2, 8), 4),
            ((3, 1), 1),
            ((3, 2), 2),
            ((3, 3), 2),
            ((3, 4), 3),
            ((3, 5), 3),
            ((3, 6), 3),
            ((3, 7), 3),
            ((3, 8), 4),
            ((3, 9), 4),
            ((3, 10), 4),
            ((3, 11), 4),
            ((3, 12), 4),
            ((3, 13), 4),
            ((3, 14), 4),
        ];

        for ((k, n), expected) in test_cases {
            assert_eq!(S::super_egg_drop(k, n), expected);
        }
    }
}
