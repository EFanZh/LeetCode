pub mod stack;

pub trait Solution {
    fn construct_array(n: i32, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [(3, 1), (3, 2), (5, 4)];

        for (n, k) in test_cases {
            let result = S::construct_array(n, k);

            assert_eq!(
                result
                    .iter()
                    .zip(result.iter().skip(1))
                    .map(|(lhs, rhs)| (lhs - rhs).abs())
                    .collect::<HashSet<_>>()
                    .len(),
                k as usize
            );

            assert!(test_utilities::unstable_sorted(result).into_iter().eq(1..=n));
        }
    }
}
