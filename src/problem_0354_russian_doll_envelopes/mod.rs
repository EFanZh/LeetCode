pub mod longest_increasing_subsequence;

pub trait Solution {
    fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[5, 4], [6, 4], [6, 7], [2, 3]] as &[_], 3),
            (&[[1, 1], [1, 1], [1, 1]], 1),
            (&[[4, 5], [4, 6], [6, 7], [2, 3], [1, 1]], 4),
        ];

        for (envelopes, expected) in test_cases {
            assert_eq!(
                S::max_envelopes(envelopes.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
