pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::option_if_let_else)]
    pub fn mask_pii(s: String) -> String {
        let mut s = s;

        let (range, mask) = if let Some(i) = s.find('@') {
            s[..1].make_ascii_lowercase();
            s[i - 1..i].make_ascii_lowercase();
            s[i + 1..].make_ascii_lowercase();

            (1..i - 1, "*****")
        } else {
            let mut s_vec = s.into_bytes();

            s_vec.retain(|c| matches!(c, b'0'..=b'9'));

            s = String::from_utf8(s_vec).unwrap();

            match s.len() {
                10 => (0..6, "***-***-"),
                11 => (0..7, "+*-***-***-"),
                12 => (0..8, "+**-***-***-"),
                _ => (0..9, "+***-***-***-"),
            }
        };

        s.replace_range(range, mask);

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mask_pii(s: String) -> String {
        Self::mask_pii(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
