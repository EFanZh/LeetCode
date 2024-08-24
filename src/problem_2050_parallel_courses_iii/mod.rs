pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[1, 3], [2, 3]] as &[_], &[3, 2, 5] as &[_]), 8),
            ((5, &[[1, 5], [2, 5], [3, 5], [3, 4], [4, 5]], &[1, 2, 3, 4, 5]), 12),
        ];

        for ((n, relations, time), expected) in test_cases {
            assert_eq!(
                S::minimum_time(n, relations.iter().map(Vec::from).collect(), time.to_vec()),
                expected,
            );
        }
    }
}
