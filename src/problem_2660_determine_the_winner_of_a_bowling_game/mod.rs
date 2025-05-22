pub mod iterative;

pub trait Solution {
    fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 10, 3, 2] as &[_], &[6, 5, 7, 3] as &[_]), 1),
            ((&[3, 5, 7, 6], &[8, 10, 10, 2]), 2),
            ((&[2, 3], &[4, 1]), 0),
            ((&[1, 1, 1, 10, 10, 10, 10], &[10, 10, 10, 10, 1, 1, 1]), 2),
        ];

        for ((player1, player2), expected) in test_cases {
            assert_eq!(S::is_winner(player1.to_vec(), player2.to_vec()), expected);
        }
    }
}
