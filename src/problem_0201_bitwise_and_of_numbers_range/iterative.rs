pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        let mut i = 0;

        while m != n {
            m >>= 1;
            n >>= 1;
            i += 1;
        }

        m << i
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
