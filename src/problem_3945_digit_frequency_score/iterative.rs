pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn digit_frequency_score(n: i32) -> i32 {
        let mut n = n.cast_unsigned();
        let mut result = 0;

        while n != 0 {
            result += n % 10;
            n /= 10;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn digit_frequency_score(n: i32) -> i32 {
        Self::digit_frequency_score(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
