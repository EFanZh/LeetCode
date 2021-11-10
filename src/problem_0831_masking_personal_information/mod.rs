pub mod iterative;

pub trait Solution {
    fn mask_pii(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("LeetCode@LeetCode.com", "l*****e@leetcode.com"),
            ("AB@qq.com", "a*****b@qq.com"),
            ("1(234)567-890", "***-***-7890"),
            ("86-(10)12345678", "+**-***-***-5678"),
            ("+86(88)1513-7-74", "+*-***-***-3774"),
            ("+(501321)-50-23431", "+***-***-***-3431"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::mask_pii(s.to_string()), expected);
        }
    }
}
