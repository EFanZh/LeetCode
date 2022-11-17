pub mod gray_code;

pub trait Solution {
    fn circular_permutation(n: i32, start: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, 3), (3, 2)];

        for (n, start) in test_cases {
            let mut result = S::circular_permutation(n, start);

            assert_eq!(result.len(), usize::pow(2, n as _));
            assert_eq!(result[0], start);

            let mut prev = *result.last().unwrap() as u32;

            for &value in &result {
                let value = value as u32;

                assert!((value ^ prev).is_power_of_two());

                prev = value;
            }

            result.sort_unstable();

            assert!(result.into_iter().eq(0..i32::pow(2, n as _)));
        }
    }
}
