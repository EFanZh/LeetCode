pub mod counting_sort;

pub trait Solution {
    fn sort_vowels(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("lEetcOde", "lEOtcede"), ("lYmpH", "lYmpH")];

        for (s, expected) in test_cases {
            assert_eq!(S::sort_vowels(s.to_string()), expected);
        }
    }
}
