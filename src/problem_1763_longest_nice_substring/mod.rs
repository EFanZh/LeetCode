pub mod divide_and_conquer;

pub trait Solution {
    fn longest_nice_substring(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("YazaAay", "aAa"), ("Bb", "Bb"), ("c", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_nice_substring(s.to_string()), expected);
        }
    }
}
