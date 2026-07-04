pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        let mut s = s;
        let n = s.trim_end_matches(['a', 'e', 'i', 'o', 'u']).len();

        s.truncate(n);

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn trim_trailing_vowels(s: String) -> String {
        Self::trim_trailing_vowels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
