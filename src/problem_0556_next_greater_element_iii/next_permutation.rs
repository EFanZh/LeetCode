pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn get_digits(mut num: u32, buffer: &mut [u8; 10]) -> &mut [u8] {
        if num == 0 {
            &mut buffer[..1]
        } else {
            let mut i = 0;

            loop {
                buffer[i] = (num % 10) as _;
                num /= 10;
                i += 1;

                if num == 0 {
                    break;
                }
            }

            &mut buffer[..i]
        }
    }

    pub fn next_greater_element(n: i32) -> i32 {
        let mut buffer = [0_u8; 10];
        let buffer = Self::get_digits(n as _, &mut buffer);

        if let Some(t) = buffer.iter().zip(&buffer[1..]).position(|(left, right)| left > right) {
            let digit_index = t + 1;
            let digit = buffer[digit_index];
            let other_index = buffer[..digit_index].partition_point(|&v| v <= digit);

            buffer.swap(digit_index, other_index);

            buffer[..digit_index].reverse();

            let mut result = 0_u32;

            for &digit in buffer.iter().rev() {
                if let Some(next_result) = result.checked_mul(10).and_then(|x| x.checked_add(u32::from(digit))) {
                    result = next_result;
                } else {
                    return -1;
                }
            }

            result.try_into().unwrap_or(-1)
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_greater_element(n: i32) -> i32 {
        Self::next_greater_element(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
