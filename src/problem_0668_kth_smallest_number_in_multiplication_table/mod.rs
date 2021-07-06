pub mod binary_search;

pub trait Solution {
    fn find_kth_number(m: i32, n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 3, 5), 3), ((2, 3, 6), 6), ((41, 31, 777), 351)];

        for ((m, n, k), expected) in test_cases {
            let result = S::find_kth_number(m, n, k);

            assert_eq!(
                result,
                expected,
                "Expect {:?} => {:?}, got {:?}.",
                (m, n, k),
                expected,
                result
            );
        }
    }
}
