pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn less_than_thousand(num: i32, result: &mut String) {
        const SINGLES: [&str; 19] = [
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        const TENS: [&str; 8] = [
            "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        match num {
            1..=19 => result.push_str(SINGLES[(num - 1) as usize]),
            20..=99 => {
                result.push_str(TENS[(num / 10 - 2) as usize]);

                let remainder = num % 10;

                if remainder != 0 {
                    result.push(' ');
                    Self::less_than_thousand(remainder, result);
                }
            }
            _ => {
                Self::less_than_thousand(num / 100, result);

                result.push_str(" Hundred");

                let remainder = num % 100;

                if remainder != 0 {
                    result.push(' ');
                    Self::less_than_thousand(remainder, result);
                }
            }
        }
    }

    pub fn number_to_words(mut num: i32) -> String {
        let mut result = String::new();

        if num == 0 {
            result.push_str("Zero");
        } else {
            for &(name, base) in &[("Billion", 1_000_000_000), ("Million", 1_000_000), ("Thousand", 1_000)] {
                if num >= base {
                    Self::less_than_thousand(num / base, &mut result);
                    result.push(' ');
                    result.push_str(name);

                    num %= base;

                    if num == 0 {
                        return result;
                    }

                    result.push(' ');
                }
            }

            Self::less_than_thousand(num, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_to_words(num: i32) -> String {
        Self::number_to_words(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
