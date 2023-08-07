pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut n = n;

        n ^= n >> 16;
        n ^= n >> 8;
        n ^= n >> 4;
        n ^= n >> 2;
        n ^= n >> 1;

        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_one_bit_operations(n: i32) -> i32 {
        Self::minimum_one_bit_operations(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
