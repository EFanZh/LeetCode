pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_zeros(num: i64) -> i64 {
        let mut num = num.cast_unsigned();
        let mut result = 0;
        let mut base = 1;

        while num != 0 {
            let digit = num % 10;

            num /= 10;

            if digit != 0 {
                result += base * digit;
                base *= 10;
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_zeros(num: i64) -> i64 {
        Self::remove_zeros(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
