pub mod greedy;

pub trait Solution {
    fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 7, 9] as &[_], &[8, 2, 5, 8] as &[_]), 2),
            ((&[1, 1, 1], &[10]), 1),
        ];

        for ((players, trainers), expected) in test_cases {
            assert_eq!(
                S::match_players_and_trainers(players.to_vec(), trainers.to_vec()),
                expected,
            );
        }
    }
}
