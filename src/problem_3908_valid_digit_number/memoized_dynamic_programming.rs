pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn valid_digit(n: i32, x: i32) -> bool {
        let mut n = n.cast_unsigned();
        let x = x.cast_unsigned();
        let mut digit;

        loop {
            digit = n % 10;
            n /= 10;

            if n == 0 {
                return false;
            }

            if digit == x {
                break;
            }
        }

        loop {
            digit = n % 10;
            n /= 10;

            if n == 0 {
                return digit != x;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_digit(n: i32, x: i32) -> bool {
        Self::valid_digit(n, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
