pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn replace_max(digits: &[u8]) -> u32 {
        let mut iter = digits.iter().copied();
        let mut result = 0;

        for digit in iter.by_ref() {
            result = result * 10 + 9;

            if digit != 9 {
                let replace = digit;

                for mut digit in iter {
                    if digit == replace {
                        digit = 9;
                    }

                    result = result * 10 + u32::from(digit);
                }

                break;
            }
        }

        result
    }

    fn replace_min(digits: &[u8]) -> u32 {
        let mut iter = digits.iter().copied();
        let first = iter.next().unwrap();
        let mut result = 1;

        let (replace, replace_with) = if first == 1 {
            let replace = loop {
                if let Some(digit) = iter.next() {
                    result *= 10;

                    if digit < 2 {
                        result += u32::from(digit);
                    } else {
                        break digit;
                    }
                } else {
                    return result;
                }
            };

            (replace, 0)
        } else {
            (first, 1)
        };

        for mut digit in iter {
            if digit == replace {
                digit = replace_with;
            }

            result = result * 10 + u32::from(digit);
        }

        result
    }

    pub fn max_diff(num: i32) -> i32 {
        let mut num = num as u32;
        let mut digits = [0_u8; 10];
        let mut i = 10;

        while num != 0 {
            i -= 1;
            digits[i] = (num % 10) as _;
            num /= 10;
        }

        let digits = &digits[i..];

        (Self::replace_max(digits) - Self::replace_min(digits)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_diff(num: i32) -> i32 {
        Self::max_diff(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
