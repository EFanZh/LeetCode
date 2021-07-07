pub mod iterative;

pub trait Solution {
    fn maximum_swap(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 21),
            (13, 31),
            (2736, 7236),
            (9973, 9973),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::maximum_swap(num), expected);
        }
    }
}
