pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let n = i64::from(n);
        let mut i = 5;
        let mut result = 0;

        while i <= n {
            result += n / i;

            i *= 5;
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn trailing_zeroes(n: i32) -> i32 {
        Self::trailing_zeroes(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
