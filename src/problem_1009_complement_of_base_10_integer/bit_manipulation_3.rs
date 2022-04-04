pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            !n << n.leading_zeros() >> n.leading_zeros()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn bitwise_complement(n: i32) -> i32 {
        Self::bitwise_complement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
