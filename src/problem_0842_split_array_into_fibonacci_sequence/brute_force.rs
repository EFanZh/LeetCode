pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn iter_integer_prefixes(first: u8, s: &[u8], mut callback: impl FnMut(i32, &[u8]) -> bool) -> bool {
        if first == 0 {
            return callback(0, s);
        }

        let mut iter = s.iter();
        let mut value = i32::from(first);

        loop {
            if callback(value, iter.as_slice()) {
                return true;
            }

            if let Some(next_value) = iter
                .next()
                .and_then(|&digit| value.checked_mul(10)?.checked_add(digit.into()))
            {
                value = next_value;
            } else {
                break;
            }
        }

        false
    }

    fn get_digits(mut x: i32, buffer: &mut [u8; 10]) -> &[u8] {
        let mut i = 10;

        while x != 0 {
            i -= 1;
            buffer[i] = (x % 10) as _;
            x /= 10;
        }

        if i == 10 {
            i = 9;
            buffer[i] = 0;
        }

        &buffer[i..]
    }

    fn check(mut first: i32, mut second: i32, mut s: &[u8], buffer: &mut [u8; 10], result: &mut Vec<i32>) -> bool {
        if let Some(third) = first.checked_add(second) {
            let mut digits = Self::get_digits(third, buffer);

            if s.starts_with(digits) {
                result.push(third);
                first = second;
                second = third;
                s = &s[digits.len()..];

                loop {
                    if s.is_empty() {
                        return true;
                    }

                    if let Some(third) = first.checked_add(second) {
                        digits = Self::get_digits(third, buffer);

                        if s.starts_with(digits) {
                            result.push(third);
                            first = second;
                            second = third;
                            s = &s[digits.len()..];
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        result.truncate(2);

        false
    }

    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut num = num.into_bytes();

        for digit in &mut num {
            *digit -= b'0';
        }

        let mut result = Vec::new();
        let mut buffer = [0_u8; 10];
        let (&digit_1, rest) = num.split_first().unwrap();

        Self::iter_integer_prefixes(digit_1, rest, |first, s| {
            s.split_first().map_or(false, |(&digit_2, rest)| {
                result.push(first);

                let success = Self::iter_integer_prefixes(digit_2, rest, |second, s| {
                    result.push(second);

                    let success = Self::check(first, second, s, &mut buffer, &mut result);

                    if !success {
                        result.pop();
                    }

                    success
                });

                if !success {
                    result.pop();
                }

                success
            })
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_into_fibonacci(num: String) -> Vec<i32> {
        Self::split_into_fibonacci(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
