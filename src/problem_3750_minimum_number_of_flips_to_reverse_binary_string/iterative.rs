pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let mut reversed = 0;
        let mut x = n;

        while x != 0 {
            reversed = (reversed << 1) | (x & 1);
            x >>= 1;
        }

        (n ^ reversed).count_ones().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_flips(n: i32) -> i32 {
        Self::minimum_flips(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
