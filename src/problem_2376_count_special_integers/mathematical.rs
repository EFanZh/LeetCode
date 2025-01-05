pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        const PERMUTATIONS: [u32; 100] = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A(0, i).
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, // A(1, i).
            1, 2, 2, 0, 0, 0, 0, 0, 0, 0, // A(2, i).
            1, 3, 6, 6, 0, 0, 0, 0, 0, 0, // A(3, i).
            1, 4, 12, 24, 24, 0, 0, 0, 0, 0, // A(4, i).
            1, 5, 20, 60, 120, 120, 0, 0, 0, 0, // A(5, i).
            1, 6, 30, 120, 360, 720, 720, 0, 0, 0, // A(6, i).
            1, 7, 42, 210, 840, 2_520, 5_040, 5_040, 0, 0, // A(7, i).
            1, 8, 56, 336, 1_680, 6_720, 20_160, 40_320, 40_320, 0, // A(8, i).
            1, 9, 72, 504, 3_024, 15_120, 60_480, 181_440, 362_880, 362_880, // A(9, i).
        ];

        const COUNTS: [u32; 10] = [
            9, 90, 738, 5_274, 32_490, 168_570, 712_890, 2_345_850, 5_611_770, 8_877_690,
        ];

        let n = n as u32;
        let mut digits = [0_u8; 10];
        let mut i = 10;
        let mut x = n;

        while x != 0 {
            let digit = x % 10;

            x /= 10;

            i -= 1;
            digits[i] = digit as _;
        }

        let digits = &digits[i..];
        let n_digits = digits.len();

        (if n_digits < 2 {
            n
        } else {
            let mut iter = digits.iter().copied();
            let first_digit = iter.next().unwrap();
            let partial_count = COUNTS[n_digits - 2];
            let mut index = 89 + n_digits;
            let mut result = partial_count + PERMUTATIONS[index] * u32::from(first_digit - 1);
            let mut used_digits = 1_u16 << first_digit;

            loop {
                if let Some(digit) = iter.next() {
                    let probe = 1 << digit;
                    let available_digits = ((1 << digit) - 1) & !used_digits;

                    index -= 11;
                    result += PERMUTATIONS[index] * available_digits.count_ones();

                    if used_digits & probe == 0 {
                        used_digits |= probe;
                    } else {
                        break;
                    }
                } else {
                    result += 1;

                    break;
                }
            }

            result
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_special_numbers(n: i32) -> i32 {
        Self::count_special_numbers(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
