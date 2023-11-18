pub mod brute_force;

pub trait Solution {
    fn tictactoe(moves: Vec<Vec<i32>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]] as &[_], "A"),
            (&[[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]], "B"),
            (
                &[[0, 0], [1, 1], [2, 0], [1, 0], [1, 2], [2, 1], [0, 1], [0, 2], [2, 2]],
                "Draw",
            ),
            (&[[0, 0], [1, 1]], "Pending"),
        ];

        for (moves, expected) in test_cases {
            assert_eq!(S::tictactoe(moves.iter().map(Vec::from).collect()), expected);
        }
    }
}
