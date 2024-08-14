pub mod dfs;

pub trait Solution {
    fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 1, 3], [6, 1, 4]] as &[_], 2),
            (&[[1, 1, 5], [10, 10, 5]], 1),
            (&[[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]], 5),
        ];

        for (bombs, expected) in test_cases {
            assert_eq!(S::maximum_detonation(bombs.iter().map(Vec::from).collect()), expected);
        }
    }
}
