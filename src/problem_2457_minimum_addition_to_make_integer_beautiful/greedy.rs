pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        let n = n as u64;
        let target = target as u8;
        let mut x = n;
        let mut digits_buffer = [0_u8; 12];
        let mut i = digits_buffer.len();

        while x != 0 {
            i -= 1;
            digits_buffer[i] = (x % 10) as u8;
            x /= 10;
        }

        let digits = &digits_buffer[i..];
        let last_non_zero_index = digits.iter().rposition(|&digit| digit != 0).unwrap();
        let (left, right) = digits.split_at(last_non_zero_index);
        let mut consecutive_nines = 0_u8;
        let mut sum = 0;
        let mut exponent = digits.len() as u32;

        for &digit in left {
            sum += digit;

            if digit == 9 {
                consecutive_nines += 1;
            } else {
                consecutive_nines = 0;
            }

            if target < sum + 1 - 9 * consecutive_nines {
                let base = u64::pow(10, exponent);

                return (base - n % base) as _;
            }

            exponent -= 1;
        }

        (if target < sum + right[0] {
            let base = u64::pow(10, exponent);

            base - n % base
        } else {
            0
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        Self::make_integer_beautiful(n, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
