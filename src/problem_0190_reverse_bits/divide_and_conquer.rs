pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut n = n;

        n = (n >> 16) | (n << 16);
        n = ((n & 0xff00_ff00) >> 8) | ((n & 0x00ff_00ff) << 8);
        n = ((n & 0xf0f0_f0f0) >> 4) | ((n & 0x0f0f_0f0f) << 4);
        n = ((n & 0xcccc_cccc) >> 2) | ((n & 0x3333_3333) << 2);
        n = ((n & 0xaaaa_aaaa) >> 1) | ((n & 0x5555_5555) << 1);

        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_bits(n: u32) -> u32 {
        Self::reverse_bits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
