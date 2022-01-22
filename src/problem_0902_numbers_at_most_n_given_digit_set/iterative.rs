pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_mapping(digits: &[String]) -> [u8; 10] {
        let mut result = [0_u8; 10];
        let mut iter = digits.iter().map(|digit| usize::from(digit.as_bytes()[0] - b'0'));
        let mut start = iter.next().unwrap();
        let mut count = 1;

        for end in iter {
            for target in &mut result[start..end] {
                *target = count;
            }

            start = end;
            count += 1;
        }

        for target in &mut result[start..] {
            *target = count;
        }

        result
    }

    fn get_digits(mut num: u32, buffer: &mut [u8; 10]) -> &[u8] {
        let mut i = 10;

        while num != 0 {
            i -= 1;
            buffer[i] = (num % 10) as _;
            num /= 10;
        }

        &buffer[i..]
    }

    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let base = digits.len() as u32;
        let mapping = Self::get_mapping(&digits);
        let mut buffer = [0; 10];
        let n_digits = Self::get_digits(n as _, &mut buffer);
        let mut result = 1;
        let mut exp = 1;

        for _ in 1..n_digits.len() {
            exp *= base;
            result += exp;
        }

        for &digit in n_digits {
            let digit = usize::from(digit);
            let mapped = mapping[digit];

            if mapped == 0 {
                result -= 1;

                break;
            } else if mapped == mapping[digit - 1] {
                result += exp * u32::from(mapped) - 1;

                break;
            }

            result += exp * u32::from(mapped - 1);
            exp /= base;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        Self::at_most_n_given_digit_set(digits, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
