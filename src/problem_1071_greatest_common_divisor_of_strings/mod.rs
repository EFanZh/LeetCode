pub mod even_odd;

pub trait Solution {
    fn gcd_of_strings(str1: String, str2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ABCABC", "ABC"), "ABC"),
            (("ABABAB", "ABAB"), "AB"),
            (("LEET", "CODE"), ""),
        ];

        for ((str1, str2), expected) in test_cases {
            assert_eq!(S::gcd_of_strings(str1.to_string(), str2.to_string()), expected);
        }
    }
}
