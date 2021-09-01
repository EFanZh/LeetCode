pub mod binomial;
pub mod dynamic_programming;

pub trait Solution {
    fn unique_paths(m: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 7), 28),
            ((3, 2), 3),
            ((7, 3), 28),
            ((3, 3), 6),
            ((51, 9), 1_916_797_311),
        ];

        for ((m, n), expected) in test_cases {
            assert_eq!(S::unique_paths(m, n), expected);
        }
    }
}
