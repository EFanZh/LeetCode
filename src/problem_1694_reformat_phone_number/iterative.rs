pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut number = number.into_bytes();

        number.retain(u8::is_ascii_digit);

        let mut iter = number.iter().copied();
        let mut result = Vec::new();

        while iter.len() > 4 {
            result.extend(iter.by_ref().take(3));
            result.push(b'-');
        }

        if iter.len() == 4 {
            result.extend(iter.by_ref().take(2));
            result.push(b'-');
        }

        result.extend(iter);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reformat_number(number: String) -> String {
        Self::reformat_number(number)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
