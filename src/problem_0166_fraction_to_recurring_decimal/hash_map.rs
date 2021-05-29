pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = i64::from(numerator);
        let denominator = i64::from(denominator);
        let integer_part = numerator / denominator;
        let mut remainder = numerator % denominator;

        let mut result = if integer_part == 0 && if denominator < 0 { numerator > 0 } else { numerator < 0 } {
            String::from("-0")
        } else {
            integer_part.to_string()
        };

        if remainder != 0 {
            let denominator = denominator.abs();
            let mut remainder_to_index = HashMap::new();

            result.push('.');
            remainder = remainder.abs();

            loop {
                match remainder_to_index.entry(remainder) {
                    Entry::Occupied(entry) => {
                        result.insert(*entry.into_mut(), '(');
                        result.push(')');

                        break;
                    }
                    Entry::Vacant(entry) => {
                        let temp = remainder * 10;

                        entry.insert(result.len());
                        result.push(char::from(b'0' + (temp / denominator) as u8));
                        remainder = temp % denominator;
                    }
                }

                if remainder == 0 {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
