pub mod recursive;

pub trait Solution {
    fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["# #", "  #", "#c "] as &[_], "abc"), true),
            ((&[" #a", " #c", " #a"], "ac"), false),
            ((&["# #", "  #", "# c"], "ca"), true),
            ((&[" #o tmo # "], "octmor"), true),
            ((&["# #", "# #", "# c"], "ca"), true),
            ((&["  ", "  "], "a"), false),
            ((&[" cz", "###"], "cz"), false),
            (
                (
                    &[
                        " ", " ", " ", " ", "#", "#", " ", "b", "#", " ", " ", " ", " ", " ", " ", " ", " ", "c", " ",
                        "#",
                    ],
                    "ba",
                ),
                true,
            ),
            ((&["c #", "###"], "ab"), false),
            ((&[" ", "#", "o", " ", "t", "m", "o", " ", "#", " "], "octmor"), true),
        ];

        for ((board, word), expected) in test_cases {
            assert_eq!(
                S::place_word_in_crossword(
                    board.iter().map(|row| row.chars().collect()).collect(),
                    word.to_string(),
                ),
                expected,
            );
        }
    }
}
