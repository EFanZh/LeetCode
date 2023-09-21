pub mod iterative;

pub trait Solution {
    fn maximum_population(logs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1993, 1999], [2000, 2010]] as &[_], 1993),
            (&[[1950, 1961], [1960, 1971], [1970, 1981]], 1960),
        ];

        for (logs, expected) in test_cases {
            assert_eq!(
                S::maximum_population(logs.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
