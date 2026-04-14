pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut n = n.cast_unsigned();
        let digits = &mut [0_u8; 10][..=n.ilog10() as usize];

        for digit in &mut *digits {
            *digit = (n % 10) as _;
            n /= 10;
        }

        digits.select_nth_unstable_by(1, |lhs, rhs| rhs.cmp(lhs));

        i32::from(digits[0] * digits[1])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product(n: i32) -> i32 {
        Self::max_product(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
