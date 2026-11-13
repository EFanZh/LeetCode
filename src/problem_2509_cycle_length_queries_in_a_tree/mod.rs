pub mod mathematical;

pub trait Solution {
    fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[5, 3], [4, 7], [2, 3]] as &[_]), &[4, 5, 3] as &[_]),
            ((2, &[[1, 2]]), &[2]),
        ];

        for ((n, queries), expected) in test_cases {
            assert_eq!(
                S::cycle_length_queries(n, queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
