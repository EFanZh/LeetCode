pub mod parse_and_expand;

pub trait Solution {
    fn brace_expansion_ii(expression: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("{a,b}{c,{d,e}}", &["ac", "ad", "ae", "bc", "bd", "be"] as &[_]),
            ("{{a,z},a{b,c},{ab,z}}", &["a", "ab", "ac", "z"]),
        ];

        for (expression, expected) in test_cases {
            assert_eq!(S::brace_expansion_ii(expression.to_string()), expected);
        }
    }
}
