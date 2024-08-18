pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn has_valid_path(grid: Vec<Vec<char>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const EXTRA_TEST_CASE_1: (&[&str], bool) = (
        &[
            "(()((()))))(()))()())))())((())))())((())))))(())(()())(()))()())()())",
            "())((())((()))()(((()()((())(()(()))())))())())))(()((()((()()(()()))(",
            "()())())())()(())))(())(((((()()())()()(())()))))(()()))()))()))()()((",
            "()()())()()))(()()(())(())()(()()(()((()(((()(()(((())()))(())))))())(",
            "))(()))())()))(((())(()((()()())()()))))(())()(())(((((()))())))()(()(",
            ")(()()))((()()()))))((())())(())())()((((((()(((((()))(())())(((()())(",
            "())))()()()))()((()(())((())()(())())(()()())())()((()()())(()))())(((",
            ")))(()()((()))))(()(()))(((((()(()()(())((()()()()((()((())))()))())()",
            ")()()()(()))((())((())())(((())())))())(()))(((()(())(())())(()))())))",
            "(((()))(())(()()(((()(((()(())()))(()()((())))))(()))((())))())((()))(",
            "))))))(()))))((())(())(())))()(()(()((()()(())))))()()))))(()(((()((()",
            ")()((()))(((())))()())(((((()()()(()())((())((()())())(((()))((()())((",
            "()()))()())((()))()(())))(()))(())(())(()(()))))()())((((()))()(((((()",
            "((())()(())))))((((((((())(())())))))((((())(()(())(()))(())((()))()((",
            "())()))(())()))(()())))()))))))))(((((()((()(()))()()(()(())()(()))(((",
            "((()(()())(()))(()(()())())(()(()((()(())(()))()))(()(()())))((((()())",
            ")()())()(()((()((()))()))(((())))))()())((())(()((((()))))()(())(())()",
            ")(())())))()()(((())((()(((())(()())))))(())())(((((()((()(()(())))(()",
            "((()()()()(()))())((()((()(()()))())(()))(((())()(()(()(((()())(((()((",
            ")())(()))(())))()((()))))))()())(()(()(())()((()())))())((())))(()()()",
            ")()())()((()()())())()()(()))((())()))()()()(((((())()))()))(((()()())",
            ")((())())()((()(()(()())((())))))()(()))()(()((((((()((((()(()(((())((",
            ")())))))((()()()))))))))()()(((((())(((()())()()))(())))))))())((()(((",
            ")())(()()()))(()()(()()(((()()(())()()()())(())))(())(()))()()))()()))",
            ")(((()((())()))(()()(((())())(()))()()()(())()(()())()())(()((()))))))",
            "(()((()())()))()))()((()()()(((())()((((())))))(((()()((())()(()(())((",
            "())))()()))()()))()(()))))((()))((()()()))((())((()))))))((()()((())))",
            "))(())()()()(())()(()))()))())))))()))))))(()))())()))())((())((()())(",
            "))))((())(()())())()(()(()((()))()()(())()))))(())()))(()((()())(()(()",
            ")))()(()())()())))()())))(((()()()()()())())))))((()))(((()()(()((()((",
            "()))((((((()))))())())(((((()((())))(())(()))))()))(((())()(()())((())",
            "(()()))(())))(()()((()()()))()((()()()())()()((((())(()()((((()(()))))",
            ")))()))))()((())))((((())()()())())(((()())(())())())))))(((()()())()(",
            "())()))))()())((())))()()((()))(())))()()((()))(((()((()()()()()))(())",
            "((())))((())((()())))()()))()((()()))))()((())()(()))()()(()))()))))))",
            "()(())))()((()(())(((()))())(((()()((()(((((()(())((())((()()((())))((",
            "()()))()(()(())))()((()))((()()()))()()())()))(())((((())()())))()))))",
            "())((((())))((((((()()(((())()(((()()))))()))(())()(())()))(()()()))((",
            "))((((((()()(())(((((()()()(()(()(()))))())(())())())(((()((()())))(()",
            ")(((()()))())(((())()(()()(()))())))))(()()())))())(()()))(())(())))()",
            "()((()()()(())(())((()))())((()))))((((()(()))))((()))())((())(())))))",
            "((())))(()(()))(())())(())((()((()((())()())()))))))(()()))(()((()))()",
            "()()((()()(((((()))))((())(()))((()(())())())()))))())(()()))())(((()(",
            "((())(())))))()(((()))))(()))()()))(()())(())())((()()))))()()(())()()",
            ")(()((())(()()()())()(()()(())()))(()(())()))()))())((((()((()(()((())",
            "()))())()((((((((())))()()()(()()()()(()()))()(((()())((())((()))))()(",
            "))))))((()(())(()(()))()((()((())())()(((())()()()))))((()(()))())()()",
            "())((()((()()())(()())))()())))(())(())))()(((()(((())())()()(()())(()",
            "))(()()())()((((()()))())((()))))(())()(())()))))(())()))))())())((())",
            ")((()()))()((())((()))))(((()))(((((((()())))(()))((()()()())(())())()",
            "(())((()(()((())))))))(((())((())((((((())()(()()(())(()()))((((((((()",
            "))))((()(())())()))())(((())(()))((()())())()))()(()()(()))()()())()))",
            ")))())()()()()((())(()))))()()))()(()())(((((())()))())))(()((((((()()",
            "))())((()()((((()(()))())())())())))())(())()())()))))(())(()()())((((",
            ")()()))(((()((())))))()(()()(((((((())())(((((()))())(())()()()()())((",
            ")((((()(()(()))())(()))())()()()))(())(())))))()())))))()(()()(()))()(",
            "()(()())())(((((()(((()(((((((()()()()))((()()))(()()()()))()(())))(()",
            "())(()))))(())((()()(())())))(()))))()(()(()()())(((())))))))(())()(((",
            ")(()(((()())())))((())((()))))())))(()))((())(((())))))(((((()()(()(((",
            ")()()(()())()(())))(((()))())))())())()))))(())()))()))))(())))(()))))",
        ],
        false,
    );

    const EXTRA_TEST_CASE_2: (&[&str], bool) = (
        &[
            "(())()())()()()()(()()))))(()((((()))))()())()()(((())())())))))()()(())()(()())))()(((",
            "(())(()(((()))(()())()(()))))()(()())()(((((()))())()(((((()))))()(()()(())((()))())(()",
            "(()(()()((()((()((())))))))())(()(()))))()))()(())((()()(((())(()))()((())()()))(()(())",
            ")((()()))()(())())(()(()))))(((()())))(((((())(((()(((()()))(()(((((((()(((()((()()))))",
            "()))))())))((()()()))())((((((()))))()((()((((((())))))(((((((())))(())(())))()))))())(",
            ")(()((((()((((((()))()())()))(()()()())((()())((())()(())(()())()())((((())(()))))(((()",
            "()))()(((()((()())()(()()()))()()(((((())(()))))(())))((()))(((()))()(()(()(()())())()(",
            "(()()()(()())()())()))())()(()((())(())()()()()((())))))(()())))(()()()))(((()))()(()()",
            "())(()((((((())))))))(()()((((()(())))))()()((())()))))())(((((()()((()))(((()(()((()((",
            "(()()((()()(((())(()(((((((((())())()))))()())(((())()())())(()(())())()))))))((((())()",
            "()(()))(())())))()(()))))()())(((()))(((((()(()))(()(())(()(())(((()()((())(()))()))))(",
            "())()()(()))())()(((()((())(()()()(()))())()(((())((())()())))()((()((((((())((((()()()",
            "()))()()(()(()()()))()((()())))))(())(())))(()))()))))()((())()(((((())()()))))())((()(",
            "()()()()(((()))())))))((()))))((()))((()()())(())((((()())()())(()()()()))()((((((()))(",
            "((((())))((())))()((()((()((()())()((()()))())(()(((()(())))(())()))))((((())))(((()(((",
            ")(()))(())()(()(())())()))()((((())()((()()())(((()())))((((((())()((()())(((()()()(())",
            "()()())(((()()((((()(((())(((())()))((())()())())())()(()))())()))()(()(((()))((()((()(",
            "(())())(()(())(()()(((()((()(()(()()(())((()((((()())))((()()))((((()((())()))((()(()((",
            "()))()(((((()()))))))((((())))()())((())))(()(())(((())))()()))())((())((()((((()))((()",
            "())())(()((())())(()))()(()(()((())()(((((())(()(((()()))((((((()))())))())(()(((()(()(",
            ")()())(())))()()((((((()))))((((((())((())(()()((((()))())()((())((((())))(()(()((()()(",
            "()(()((())))))))((()((()()))((())(()))))(())(())(())))))()(())((((())(()(()()()(()))())",
            "())()))()))()(()((())))()((()()(()))())()()))())(()()))()())(()()()((()((()))()))(((())",
            "()()()()(()()))()))()))))(()))))))()()(()((((()))()(())((())(())))((((()()())(()))))()(",
            "())))(())((()()(()())(()())))))())()))))(()()))(((())(())((()())())))((()()(()))))(())(",
            "))((()()())(()))()())))(())(())((()(()(()())((()()((()))()()()))(()()((()()(()()(())())",
            "(()))))()))()())))())(((((((())())((())(()(()(())()))((())))(((((((())()))()()()))()(((",
            ")))(())((()(()((()(()()(())(())))())((((()(((()()))())(())())(()((())())())))()))))))))",
            ")))))(((((()())()()(())()())))))))()()()())(()()((((()))(((()()))((())(((())()()()())))",
            "()())(((()((()(((())))(()))())()(()(((())))))))))()(()(()())())(()())))())((((())((()()",
            "())))(((()))()(()()))(())))(()(()())((((()((())))())))())((()((()))((((((()(()((((()))(",
            "((())()))()((())))()))))((()((())()())))()))(()()))(()())()()()(()())))())(()(((((()))(",
            "()((()))()(()()))())((()(((()())()((())((())((((()(((((()))())))))))(()((())))())()())(",
            "()()))()(()())()()()))))))(())()))))())()(()())))(()(()))(()))()(((()()()(())((()))()((",
            "))()))()((()(())))))())())))()()))))()())))(((()(()(((())((((((())())()()()()()((()(())",
            "((()()((()))((((()))()(())(()))))()))(())(())))((((())(()))))()))()))(()())()(())(())))",
            ")()()((()())))()))))()())))()()()())((())(()()(()))))(()))(((())(((((()()()())(())())))",
            "))())())()(()()()()((((((((())())()(((()))((((()()))))(((()()(()(()(()())(())()(((()())",
            "((()()(()))))))()(()((()(())()(())(((()()())))(()()(())(()))))((())()))()))())((((())()",
            "()))((()()(()())()()))()()())()()(()))()))))())())())())((((((()()))()((()()))()()(())(",
            ")())(())))()()()()()))(()()(((())(())()()(((())))((()()(((((())())())(((()))()))()))()(",
            ")))(())(()(((()()()(())(()((()))((((()()))((()()))())(())()))()())(())())()()())(((()((",
            "(((()((()(((()((()(((((()())(())))))()))((())))()()(((())))())))))(()())((()())(((()(((",
            ")(()))((()))(()()())(()())()((()))()()())))))((((((())(()(())(((((()))))()(()(((()())))",
            ")(()(())(())((()(((()()))(((())))(())(())))(())))((()((()())))))(((((((()((()(()(())))(",
            ")((()()))(()((()()()()))()(())))()()()()())()())()()))((((()((()))((((()(((())(()())))(",
            "()(())((()))()((())())((()))()())()()((())(((())()(()(((((()()))((((()()(((()()())(((((",
            "()(((()(()()))(()(((()((())))))(()()()()()))()))())())(())((()))()()((()(((())())((()))",
            "()))))()()(())())))())())()))(())))())((()(()()()((()(()(()())((((((((()(((()))(())()))",
            "(())()))())))()))(())()(()()(((())()))))())())(((((((((((((()()()))(((())())()))()(((()",
            "())((())(())))((())()(()()))(()))())((()))())()))()()))()())))()()(()))((((()()((())()(",
            ")()((((())(()(()(()))(()))(())(())((((())(((()()((()())()((())()(())(()))))(((((()))(()",
            ")(((()())())())((()(((()(())(()())()))()(()()())())(())()()(())((()()()())(()(())()()()",
            "))((())((()()))((()(())(())((()())(())))))((())()(()(((())(())(((()()()))((()))))()((()",
            "()(((())(()())(()())))))))))(())((()))(()(()()())(())())((()()))))())()()))()())))(((((",
            ")(()))(((())()((()))))()(()))()((())))(()((())))()(((((())(())(())()()()(())()(()()))((",
            ")())))(()(()()))(((())))(()))(()(())))(((((())))))(()())))(()))(()))())())((()((()))()(",
            "))())(()(()()(((()))()(()(((()))(()((()))(((())())(()(()((())())()(())((()(((()))(()(()",
            "()(()()(()))((())(((()())(()((())()()((()(((()(()()(()))()(()))((()()(((((()()(()(()()(",
            ")))((()())(((((()((())(()(())))()(())(()(((()()()))(()))())(()(()(((((())())())(((())((",
            "()(()())))())(())))))(()()(()((())()()))(())))()((()((()))))()()(()()())))))())(())))))",
            "()))))(())))())))(((((((((()(()())(((((((()(())((((()()()(()((()()(()((())))(()(()))(((",
            "())((((())))((()((((((()))()))())()(())(((((()))()(()(((()))()))()(((())())((((()(())()",
            ")((()))()((()(()()(()(())))))(()(())()()((((()(())))))()(())(())()((())))(())((())()())",
            "))))(())(((((()((()()()()()))()))()()(()))(()()()())()(()((()))(()))((()())(((()))))(()",
            "((())))())(()))()))(())))(()())((()))))(((()(()))((((((()(()(()((((())())((()(((((()))(",
            ")(()()))((((())))()(()(((()()(()()())))()()()((())))))))(((())()()(((()()))()())(()))))",
            "(((()(((()((()())))(()((((())())))))))()()()())()(()(((((()()())(()()()(((()))(((()))))",
            "()())((())(()(()()(())))))((((())))))((())())()))(())))))()(())(()))())()()(())((()(())",
            "))(()))(()(()(()()()()()))()())))()()())))))(((((((()(((()))))()()())()(((()))))((()()(",
            ")()()(((())))()((()))(()(())((((()))()())(()())())(((())))()())()(()())((())))))(((())(",
            "(())((((()))))(((()(((())())()(()()()()()()()())())))()((((()((()()())())((()())()()))(",
            "(()())()()((())()(()()))(((())()))))(()((((()((())()()())((()((()))((((((()()((((()((((",
            "((())()(((()))()()))))()(((((((()())())(((((()))())()(((())))()((()()()()((()(()(()(())",
            ")))))())()()))))(())(((()()())())(()))()()(()(()((())()))((((((())(())()()))()()()))(()",
            "))()))())))()))(())(())(((()()))))())()))((((()(())))(()()()()()()((()(()))))((()))())(",
            ")(())()))()))(()())())()))((((())(()))((()))(()))))((())())())((())((())((())()()()()((",
            "))()())(())))))))((((((((((()(((())()())(())))(()))))()()((())()(()(()((((())(()(()))))",
        ],
        true,
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["(((", ")()", "(()", "(()"] as &[_], true),
            (&["))", "(("], false),
            (
                &["(()(()()(", ")((()))((", "())(()(((", "(()()(()(", "()()))())"],
                false,
            ),
            (
                &[
                    "()(()())()(((((",
                    "(())))((()(()()",
                    "()))(()(()(()((",
                    "(()()()))))()))",
                    ")(()())(()())((",
                    ")()((()()()()))",
                    ")((())())((())(",
                    "()))(())))(((()",
                    "))((())(())))))",
                    ")))()((()))((()",
                    "(())))(((((()((",
                    "((()()))()))())",
                    ")((())((()(()((",
                    "(()(((())()()()",
                    "(()(()()()(((((",
                    "))((()))))(())(",
                    ")))))(()(((())(",
                    ")((()())))()()(",
                    "(()()())))()(()",
                    "()(((()()))((((",
                    ")())))(()(((())",
                    "))(((()))(()(((",
                    ")(((()))())))))",
                    "((())()))((((()",
                    "(()())(()(((())",
                    "(((((())())()))",
                    ")()()))))((((()",
                    ")(((((()()()))(",
                    "((((()(((())))(",
                    ")))())(()))()))",
                ],
                true,
            ),
            (
                &[
                    "())(())(()(()(())(()()))))",
                    ")))))()()())(()()())))))))",
                    "(()))()(((())()(()((())(((",
                    ")(((()())(()))()()(()()())",
                    "(())(()(()(((()((((((((()(",
                    ")((())()))((((()))(()())((",
                    "((((())))))()())())))(()((",
                    "((()()())())(())())()(((((",
                    "))())((())()(()))()(())())",
                    "((()(()))(()))()()())(((((",
                    "(())(()()))((()(()())))()(",
                    "(((((())()))((()(()(((()()",
                    "(((()()()))())()(())((((()",
                ],
                true,
            ),
            (
                &[
                    "((((())())())()(())()())())))()((",
                    "())(())))()()()(())(((()))))(()((",
                    "))(())))()()))(()()((()))))))))((",
                    "((()()))()((()()()()()()))(()()))",
                    ")()())))(((()()((((()(())((()())(",
                    ")))()))(())(((()(())()))())())(((",
                    ")()(())()))((()((()()()))))())(()",
                    "(())())(((()()(((((()()())((()(()",
                    "))(())((((((())((())()(())())()((",
                    "))))())(((()))())))))(((())))((()",
                ],
                true,
            ),
            EXTRA_TEST_CASE_1,
            EXTRA_TEST_CASE_2,
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::has_valid_path(board.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}