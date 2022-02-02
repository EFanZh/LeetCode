pub mod iterative;

pub trait Solution {
    fn reverse_only_letters(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("ab-cd", "dc-ba"),
            ("a-bC-dEf-ghIj", "j-Ih-gfE-dCba"),
            ("Test1ng-Leet=code-Q!", "Qedo1ct-eeLg=ntse-T!"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_only_letters(s.to_string()), expected);
        }
    }
}
