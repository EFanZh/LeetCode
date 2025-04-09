pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = 0;

        loop {
            result += (n % 10) as i32;
            n /= 10;

            if n == 0 {
                break;
            }

            result -= (n % 10) as i32;
            n /= 10;

            if n == 0 {
                result = -result;

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn alternate_digit_sum(n: i32) -> i32 {
        Self::alternate_digit_sum(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
