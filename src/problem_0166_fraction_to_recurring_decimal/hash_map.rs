pub struct Solution {}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = i64::from(numerator);
        let denominator = i64::from(denominator);
        let integer_part = numerator / denominator;

        let mut result = if integer_part == 0 && if denominator < 0 { numerator > 0 } else { numerator < 0 } {
            String::from("-0")
        } else {
            integer_part.to_string()
        };

        let numerator = numerator.abs();
        let denominator = denominator.abs();
        let mut fraction_part = Vec::new();
        let mut remainder_to_index = HashMap::new();
        let mut remainder = numerator % denominator;

        loop {
            if remainder == 0 {
                if !fraction_part.is_empty() {
                    result.push('.');
                    result.extend(fraction_part.into_iter().map(char::from));
                }

                break;
            } else {
                let temp = remainder * 10;
                let digit = (temp / denominator) as u8;

                match remainder_to_index.entry(remainder) {
                    Entry::Occupied(entry) => {
                        let (left, right) = fraction_part.split_at(*entry.into_mut());

                        result.push('.');
                        result.extend(left.iter().copied().map(char::from));
                        result.push('(');
                        result.extend(right.iter().copied().map(char::from));
                        result.push(')');

                        break;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(fraction_part.len());
                        fraction_part.push(b'0' + digit);
                        remainder = temp % denominator;
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        Self::fraction_to_decimal(numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
