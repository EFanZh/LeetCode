pub mod mathematical;

pub trait Solution {
    fn count_substrings(s: String, c: char) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abada", 'a'), 6), (("zzz", 'z'), 6)];

        for ((s, c), expected) in test_cases {
            assert_eq!(S::count_substrings(s.to_string(), c), expected);
        }
    }
}
