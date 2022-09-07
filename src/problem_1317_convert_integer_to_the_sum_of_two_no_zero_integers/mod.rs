pub mod iterative;

pub trait Solution {
    fn get_no_zero_integers(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        for n in test_cases {
            let [a, b]: [_; 2] = S::get_no_zero_integers(n).try_into().unwrap();

            assert!(a > 0);
            assert!(b > 0);
            assert_eq!(a + b, n);
        }
    }
}
