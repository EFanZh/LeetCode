pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn consecutive_set_bits(n: i32) -> bool {
        (n & (n >> 1)).cast_unsigned().is_power_of_two()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn consecutive_set_bits(n: i32) -> bool {
        Self::consecutive_set_bits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
