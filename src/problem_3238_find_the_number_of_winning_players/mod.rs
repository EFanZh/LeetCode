pub mod iterative;

pub trait Solution {
    fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[0, 0], [1, 0], [1, 0], [2, 1], [2, 1], [2, 0]] as &[_]), 2),
            ((5, &[[1, 1], [1, 2], [1, 3], [1, 4]]), 0),
            ((5, &[[1, 1], [2, 4], [2, 4], [2, 4]]), 1),
            ((2, &[[1, 5], [0, 10], [1, 4]]), 1),
        ];

        for ((n, pick), expected) in test_cases {
            assert_eq!(
                S::winning_player_count(n, pick.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
