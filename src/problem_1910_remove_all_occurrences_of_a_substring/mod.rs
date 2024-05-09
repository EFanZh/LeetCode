pub mod kmp;

pub trait Solution {
    fn remove_occurrences(s: String, part: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("daabcbaabcbc", "abc"), "dab"),
            (("axxxxyyyyb", "xy"), "ab"),
            (("gjzgbpggjzgbpgsvpwdk", "gjzgbpg"), "svpwdk"),
        ];

        for ((s, part), expected) in test_cases {
            assert_eq!(S::remove_occurrences(s.to_string(), part.to_string()), expected);
        }
    }
}
