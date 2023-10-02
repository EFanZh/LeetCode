pub mod dynamic_programming;

pub trait Solution {
    fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 10, 15] as &[_], &[1, 2, 3, 4, 5] as &[_]), 34),
            ((&[4, 5, 6, 5], &[2, 1, 2, 1]), 16),
            ((&[1, 2, 3, 5], &[8, 9, 10, 1]), 6),
            (
                (
                    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    &[811, 364, 124, 873, 790, 656, 581, 446, 885, 134],
                ),
                10,
            ),
            ((&[1, 3, 7, 3, 2, 4, 10, 7, 5], &[4, 5, 2, 1, 1, 2, 4, 1, 4]), 29),
        ];

        for ((scores, ages), expected) in test_cases {
            assert_eq!(S::best_team_score(scores.to_vec(), ages.to_vec()), expected);
        }
    }
}
