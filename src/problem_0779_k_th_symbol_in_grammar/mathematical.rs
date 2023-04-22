pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(unused_variables)] // Expected.
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        ((k - 1).count_ones() & 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_grammar(n: i32, k: i32) -> i32 {
        Self::kth_grammar(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
