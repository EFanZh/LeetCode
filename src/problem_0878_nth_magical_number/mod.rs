pub mod binary_search;
pub mod mathematical;

pub trait Solution {
    fn nth_magical_number(n: i32, a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 2, 3), 2),
            ((4, 2, 3), 6),
            ((5, 2, 4), 10),
            ((1_000_000_000, 40000, 40000), 999_720_007),
            ((3, 8, 3), 8),
        ];

        for ((n, a, b), expected) in test_cases {
            assert_eq!(S::nth_magical_number(n, a, b), expected);
        }
    }
}
