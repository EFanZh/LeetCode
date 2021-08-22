pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn get_digits(mut num: u32, buffer: &mut [u8; 10]) -> &mut [u8] {
        let mut i = 0;

        while num != 0 {
            buffer[i] = (num % 10) as _;
            num /= 10;
            i += 1;
        }

        &mut buffer[..i]
    }

    fn helper(n: u32) -> u32 {
        let mut buffer = [0_u8; 10];
        let buffer = Self::get_digits(n, &mut buffer);
        let mut iter = buffer.iter().copied().enumerate().rev();

        if let Some((mut start, mut prev)) = iter.next() {
            for (i, digit) in iter {
                match digit.cmp(&prev) {
                    Ordering::Less => {
                        let mut result = 0;

                        for &digit in buffer[start + 1..].iter().rev() {
                            result = result * 10 + u32::from(digit);
                        }

                        result = result * 10 + u32::from(buffer[start] - 1);

                        for _ in 0..start {
                            result = result * 10 + 9;
                        }

                        return result;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        start = i;
                        prev = digit;
                    }
                }
            }
        }

        n
    }

    pub fn monotone_increasing_digits(n: i32) -> i32 {
        Self::helper(n as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn monotone_increasing_digits(n: i32) -> i32 {
        Self::monotone_increasing_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
