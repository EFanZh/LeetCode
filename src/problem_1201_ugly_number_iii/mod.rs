pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 2, 3, 5), 4),
            ((4, 2, 3, 4), 6),
            ((5, 2, 11, 13), 10),
            ((1_000_000_000, 2, 216_037_921, 382_899_298), 1_999_999_990),
        ];

        for ((n, a, b, c), expected) in test_cases {
            assert_eq!(S::nth_ugly_number(n, a, b, c), expected);
        }
    }
}
