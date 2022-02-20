pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut n = n as u32;
        let mut digits = [0_u8; 10];
        let mut length = 0;

        loop {
            digits[length] = (n % 10) as _;
            n /= 10;
            length += 1;

            if n == 0 {
                break;
            }
        }

        let digits = &mut digits[..length];

        digits.sort_unstable();

        match digits.len() {
            1 => matches!(digits, [1 | 2 | 4 | 8]),
            2 => matches!(digits, [1 | 4, 6] | [2, 3]),
            3 => matches!(digits, [1, 2, 5 | 8] | [2, 5, 6]),
            4 => matches!(digits, [0, 1, 2, 4] | [0, 2, 4, 8] | [0, 4, 6, 9] | [1, 2, 8, 9]),
            5 => matches!(digits, [1, 3, 4, 6, 8] | [2, 3, 6, 7, 8] | [3, 5, 5, 6, 6]),
            6 => matches!(digits, [0, 1, 1, 2, 3, 7] | [1, 2, 2, 4, 4, 6] | [2, 2, 4, 5, 8, 8]),
            7 => matches!(
                digits,
                [0, 1, 4, 5, 6, 7, 8] | [0, 1, 2, 2, 5, 7, 9] | [0, 1, 3, 4, 4, 4, 9] | [0, 3, 6, 8, 8, 8, 8]
            ),
            8 => matches!(
                digits,
                [1, 1, 2, 6, 6, 7, 7, 7] | [2, 3, 3, 3, 4, 4, 5, 5] | [0, 1, 4, 6, 6, 7, 8, 8]
            ),
            _ => matches!(
                digits,
                [1, 1, 2, 2, 3, 4, 7, 7, 8] | [2, 3, 4, 4, 5, 5, 6, 6, 8] | [0, 1, 2, 3, 5, 6, 7, 8, 9]
            ),
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reordered_power_of2(n: i32) -> bool {
        Self::reordered_power_of2(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
