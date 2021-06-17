pub mod dynamic_programming;
pub mod fast_fibonacci_iterative;
pub mod fast_fibonacci_recursive;

pub trait Solution {
    fn climb_stairs(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 5),
            (5, 8),
            (6, 13),
            (7, 21),
            (8, 34),
            (9, 55),
            (10, 89),
            (45, 1_836_311_903),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::climb_stairs(n), expected);
        }
    }
}
