pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        if matches!(num, -11..=20) {
            num
        } else {
            let (is_negative, mut num) = if num < 0 {
                (true, (-num) as u64)
            } else {
                (false, num as u64)
            };

            let mut digits_buffer = [0; 20];
            let mut i = 0;

            while num != 0 {
                digits_buffer[i] = (num % 10) as u8;
                i += 1;
                num /= 10;
            }

            let digits = &mut digits_buffer[..i];

            digits.sort_unstable();

            let mut result = 0;

            if is_negative {
                for &x in digits.iter().rev() {
                    result = result * 10 + u64::from(x);
                }

                -(result as i64)
            } else {
                let non_zero_index = digits.iter().position(|&digit| digit != 0).unwrap();

                digits.swap(0, non_zero_index);

                for &x in &*digits {
                    result = result * 10 + u64::from(x);
                }

                result as _
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_number(num: i64) -> i64 {
        Self::smallest_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
