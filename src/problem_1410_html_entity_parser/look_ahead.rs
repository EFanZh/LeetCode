pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut result = Vec::new();
        let mut bytes = text.as_bytes();

        loop {
            let (c, rest) = match bytes {
                [b'&', b'q', b'u', b'o', b't', b';', rest @ ..] => (b'"', rest),
                [b'&', b'a', b'p', b'o', b's', b';', rest @ ..] => (b'\'', rest),
                [b'&', b'a', b'm', b'p', b';', rest @ ..] => (b'&', rest),
                [b'&', b'g', b't', b';', rest @ ..] => (b'>', rest),
                [b'&', b'l', b't', b';', rest @ ..] => (b'<', rest),
                [b'&', b'f', b'r', b'a', b's', b'l', b';', rest @ ..] => (b'/', rest),
                [c, rest @ ..] => (*c, rest),
                [] => break,
            };

            result.push(c);
            bytes = rest;
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn entity_parser(text: String) -> String {
        Self::entity_parser(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
