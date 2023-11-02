pub mod dynamic_programming;
pub mod mathematical;

pub trait Solution {
    fn two_egg_drop(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // See <https://oeis.org/A002024>.
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 4),
            (8, 4),
            (100, 14),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::two_egg_drop(n), expected);
        }
    }
}
