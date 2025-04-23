pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num = num as u32;
        let mut buffer = [0_u8; 10];
        let mut i = buffer.len();
        let mut x = num;

        while x != 0 {
            i -= 1;

            buffer[i] = (x % 10) as _;
            x /= 10;
        }

        let digits = &buffer[i..];
        let min_replace = digits[0];
        let mut min_value = 0;

        for &digit in digits {
            min_value = min_value * 10 + u32::from(if digit == min_replace { 0 } else { digit });
        }

        let max_value = if let Some(&max_replace) = digits.iter().find(|&&x| x != 9) {
            let mut max_value = 0;

            for &digit in digits {
                max_value = max_value * 10 + u32::from(if digit == max_replace { 9 } else { digit });
            }

            max_value
        } else {
            num
        };

        (max_value - min_value) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_max_difference(num: i32) -> i32 {
        Self::min_max_difference(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
