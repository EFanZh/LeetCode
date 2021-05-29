pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
        while m < n {
            n &= n - 1;
        }

        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn range_bitwise_and(m: i32, n: i32) -> i32 {
        Self::range_bitwise_and(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
