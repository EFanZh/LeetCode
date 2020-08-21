pub struct Solution {}

impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        n.reverse_bits()
    }
}

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
