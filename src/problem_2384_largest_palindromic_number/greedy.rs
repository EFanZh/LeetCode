pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counts = [0_u32; 16];

        for digit in num.bytes() {
            counts[(usize::from(digit) - usize::from(b'0')) & 15] += 1;
        }

        let counts = <&[_; 10]>::try_from(&counts[..10]).unwrap();
        let mut result = num.into_bytes();

        result.clear();

        if counts.iter().rposition(|&count| count > 1).is_some_and(|i| i != 0) {
            for (digit, &count) in (b'0'..=b'9').zip(counts).rev() {
                result.extend(iter::repeat(digit).take((count / 2) as _));
            }

            if let Some(i) = counts.iter().rposition(|&count| count & 1 != 0) {
                result.push(b'0' + i as u8);
            }

            for (digit, &count) in (b'0'..=b'9').zip(counts) {
                result.extend(iter::repeat(digit).take((count / 2) as _));
            }
        } else {
            let i = counts.iter().rposition(|&count| count != 0).unwrap_or(0);

            result.push(b'0' + i as u8);
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_palindromic(num: String) -> String {
        Self::largest_palindromic(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
