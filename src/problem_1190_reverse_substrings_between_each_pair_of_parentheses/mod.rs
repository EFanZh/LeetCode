pub mod two_passes;

pub trait Solution {
    fn reverse_parentheses(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("(abcd)", "dcba"),
            ("(u(love)i)", "iloveu"),
            ("(ed(et(oc))el)", "leetcode"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_parentheses(s.to_string()), expected);
        }
    }
}
