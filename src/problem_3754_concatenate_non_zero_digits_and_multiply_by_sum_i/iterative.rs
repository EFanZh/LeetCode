pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut digits = 0;
        let mut base = 1;
        let mut digit_sum = 0;
        let mut n = n.cast_unsigned();

        while n != 0 {
            let digit = n % 10;

            n /= 10;

            if digit != 0 {
                digits += base * digit;
                base *= 10;
                digit_sum += digit;
            }
        }

        (u64::from(digits) * u64::from(digit_sum)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_and_multiply(n: i32) -> i64 {
        Self::sum_and_multiply(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
