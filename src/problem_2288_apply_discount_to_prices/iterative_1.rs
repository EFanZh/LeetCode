pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        use std::fmt::Write;

        let mut result = String::with_capacity(sentence.len() * 2);
        let discount = u64::from(100 - discount as u32);
        let mut iter = sentence.split(' ');
        let mut word = iter.next().unwrap();

        loop {
            if let Some(value) = word.strip_prefix('$').and_then(|word| word.parse::<u64>().ok()) {
                let value = value * discount;

                write!(result, "${}.{:02}", value / 100, value % 100).ok();
            } else {
                result.push_str(word);
            }

            if let Some(next_word) = iter.next() {
                result.push(' ');
                word = next_word;
            } else {
                break;
            }
        }

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
