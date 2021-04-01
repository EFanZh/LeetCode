pub mod recursive;

pub trait Solution {
    fn exist(board: Vec<Vec<char>>, word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        &['A', 'B', 'C', 'E'] as &[_],
                        &['S', 'F', 'C', 'S'],
                        &['A', 'D', 'E', 'E'],
                    ] as &[&[_]],
                    "ABCCED",
                ),
                true,
            ),
            (
                (
                    &[&['A', 'B', 'C', 'E'], &['S', 'F', 'C', 'S'], &['A', 'D', 'E', 'E']],
                    "SEE",
                ),
                true,
            ),
            (
                (
                    &[&['A', 'B', 'C', 'E'], &['S', 'F', 'C', 'S'], &['A', 'D', 'E', 'E']],
                    "ABCB",
                ),
                false,
            ),
            ((&[&['a']], "a"), true),
        ];

        for ((board, word), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::exist(board.iter().copied().map(<[_]>::to_vec).collect(), word.to_string()),
                expected
            );
        }
    }
}
