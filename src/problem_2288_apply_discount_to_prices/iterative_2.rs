pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn write_with_discount(target: &mut String, word: &str, scale: u64) {
        use std::fmt::Write;

        if let Some(suffix) = word.strip_prefix('$') {
            if let Ok(value) = suffix.parse::<u64>() {
                let multiplied = value * scale;
                let integer_part = multiplied / 100;
                let fraction_part = multiplied % 100;

                target.push('$');
                write!(target, "{integer_part}").unwrap();
                target.push('.');
                write!(target, "{fraction_part:02}").unwrap();

                return;
            }
        }

        target.push_str(word);
    }

    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let discount = u64::from(100 - discount as u32);
        let mut result = String::with_capacity(sentence.len() * 2);

        for word in sentence.split_ascii_whitespace() {
            Self::write_with_discount(&mut result, word, discount);
            result.push(' ');
        }

        result.pop();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn discount_prices(sentence: String, discount: i32) -> String {
        Self::discount_prices(sentence, discount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
