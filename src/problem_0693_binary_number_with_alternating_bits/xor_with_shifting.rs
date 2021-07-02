pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = n as u32;

        ((n ^ (n >> 1)) + 1).is_power_of_two()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_alternating_bits(n: i32) -> bool {
        Self::has_alternating_bits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
