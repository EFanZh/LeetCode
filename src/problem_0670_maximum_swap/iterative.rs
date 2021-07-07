pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn get_digits(buffer: &mut [u8; 9], mut num: i32) -> &mut [u8] {
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

    pub fn maximum_swap(num: i32) -> i32 {
        if num < 12 {
            num
        } else {
            let mut buffer = [0; 9];
            let digits = Self::get_digits(&mut buffer, num);
            let mut max_digit = 0;
            let mut max_index = 0;
            let mut swap_index = 0;
            let mut swap_with_index = 0;

            for (i, &digit) in digits.iter().enumerate() {
                match digit.cmp(&max_digit) {
                    Ordering::Less => {
                        swap_index = i;
                        swap_with_index = max_index;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        max_digit = digit;
                        max_index = i;
                    }
                }
            }

            if swap_index == 0 {
                num
            } else {
                digits.swap(swap_index, swap_with_index);

                let mut result = 0;

                for &digit in digits.iter().rev() {
                    result = result * 10 + i32::from(digit);
                }

                result
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_swap(num: i32) -> i32 {
        Self::maximum_swap(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
