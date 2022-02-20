pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn item(s: &[u8]) -> Option<(i32, &[u8])> {
        (s.first().copied() == Some(b'(')).then(|| {
            let (inner, rest) = Self::list(&s[1..]);

            (if inner == 0 { 1 } else { inner * 2 }, &rest[1..])
        })
    }

    fn list(mut s: &[u8]) -> (i32, &[u8]) {
        let mut result = 0;

        while let Some((value, rest)) = Self::item(s) {
            result += value;
            s = rest;
        }

        (result, s)
    }

    pub fn score_of_parentheses(s: String) -> i32 {
        Self::list(s.as_bytes()).0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_of_parentheses(s: String) -> i32 {
        Self::score_of_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
