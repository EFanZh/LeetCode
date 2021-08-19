pub mod iterative;

pub trait Solution {
    fn remove_comments(source: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "/*Test program */",
                    "int main()",
                    "{ ",
                    "  // variable declaration ",
                    "int a, b, c;",
                    "/* This is a test",
                    "   multiline  ",
                    "   comment for ",
                    "   testing */",
                    "a = b + c;",
                    "}",
                ] as &[_],
                &["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"] as &[_],
            ),
            (&["a/*comment", "line", "more_comment*/b"], &["ab"]),
        ];

        for (source, expected) in test_cases {
            assert_eq!(
                S::remove_comments(source.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
