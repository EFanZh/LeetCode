pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        const EVEN_MASK: u32 = 0x_55555555_u32;
        const ODD_MASK: u32 = 0x_aaaaaaaa_u32;

        let n = n as u32;
        let even_bits = (n & EVEN_MASK).count_ones();
        let odd_bits = (n & ODD_MASK).count_ones();

        vec![even_bits as _, odd_bits as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn even_odd_bit(n: i32) -> Vec<i32> {
        Self::even_odd_bit(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
