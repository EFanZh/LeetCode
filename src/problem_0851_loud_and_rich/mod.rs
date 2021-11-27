pub mod dfs;
pub mod dfs_2;

pub trait Solution {
    fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (
                &[[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]] as &[_],
                &[3, 2, 5, 4, 6, 1, 7, 0] as &[_],
            ),
            &[5, 5, 2, 5, 4, 5, 6, 7] as &[_],
        )];

        for ((richer, quiet), expected) in test_cases {
            assert_eq!(
                S::loud_and_rich(richer.iter().copied().map(Vec::from).collect(), quiet.to_vec()),
                expected
            );
        }
    }
}
