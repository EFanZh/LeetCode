pub mod quick_select_and_memoized_dynamic_programming;

pub trait Solution {
    fn get_kth(lo: i32, hi: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((12, 15, 2), 13),
            ((7, 11, 4), 7),
            ((1, 1000, 500), 606),
            ((1, 1000, 1000), 871),
        ];

        for ((lo, hi, k), expected) in test_cases {
            assert_eq!(S::get_kth(lo, hi, k), expected);
        }
    }
}
