pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn cumulative_xor(n: i32) -> i32 {
        // This function calculates:
        //
        // ```rust
        // match n & 3 {
        //     0 => n,
        //     1 => 1,
        //     2 => n + 1,
        //     _ => 0
        // }
        // ```

        let maybe_n = n & (n & 1).wrapping_sub(1);
        let maybe_1 = (n ^ (n >> 1)) & 1;

        maybe_n + maybe_1
    }

    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let i = (start >> 1) - 1;
        let j = i + n;
        let high = Self::cumulative_xor(i) ^ Self::cumulative_xor(j);
        let low = start & n & 1;

        (high << 1) | low
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn xor_operation(n: i32, start: i32) -> i32 {
        Self::xor_operation(n, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
