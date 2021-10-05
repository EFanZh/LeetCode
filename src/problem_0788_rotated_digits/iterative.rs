pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_digits(mut n: u32, buffer: &mut [u8]) -> &[u8] {
        let mut i = 0;

        loop {
            buffer[i] = (n % 10) as _;
            i += 1;
            n /= 10;

            if n == 0 {
                break;
            }
        }

        &buffer[..i]
    }

    pub fn rotated_digits(n: i32) -> i32 {
        let n = (n + 1) as _;
        let mut digits = [0; 5];
        let digits = Self::get_digits(n, &mut digits);
        let mut result = 0;

        for (i, &digit) in (0..digits.len() as u32).zip(digits).rev() {
            match digit {
                0 => {}
                1 => result += u32::pow(7, i),
                2 => result += u32::pow(7, i) * 2,
                3 | 4 => {
                    result += u32::pow(7, i) * 3;

                    break;
                }
                5 => result += u32::pow(7, i) * 3,
                6 => result += u32::pow(7, i) * 4,
                7 => {
                    result += u32::pow(7, i) * 5;

                    break;
                }
                8 => result += u32::pow(7, i) * 5,
                _ => result += u32::pow(7, i) * 6,
            }
        }

        for (i, &digit) in (0..digits.len() as u32).zip(digits).rev() {
            match digit {
                0 => {}
                1 => result -= u32::pow(3, i),
                2 | 3 | 4 | 5 | 6 | 7 => {
                    result -= u32::pow(3, i) * 2;

                    break;
                }
                8 => result -= u32::pow(3, i) * 2,
                _ => {
                    result -= u32::pow(3, i) * 3;

                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotated_digits(n: i32) -> i32 {
        Self::rotated_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
