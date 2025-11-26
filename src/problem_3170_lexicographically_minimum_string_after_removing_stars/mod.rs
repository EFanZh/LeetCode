pub mod greedy;

pub trait Solution {
    fn clear_stars(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaba*", "aab"), ("abc", "abc")];

        for (s, expected) in test_cases {
            assert_eq!(S::clear_stars(s.to_string()), expected);
        }
    }
}
