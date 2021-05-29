pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut result = 0;

        for i in 0..32 {
            result |= (n & (1 << i)) >> i << (31 - i);
        }

        result
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
