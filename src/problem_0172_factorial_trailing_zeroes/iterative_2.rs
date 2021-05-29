pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        // https://oeis.org/A027868

        let mut result = 0;

        while n != 0 {
            n /= 5;

            result += n;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
