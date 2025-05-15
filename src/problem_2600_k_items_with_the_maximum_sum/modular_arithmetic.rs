pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn helper(num_ones: u32, num_zeros: u32, k: u32) -> u32 {
        k.min(num_ones) - k.saturating_sub(num_ones + num_zeros)
    }

    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
        _ = num_neg_ones;

        Self::helper(num_ones as _, num_zeros as _, k as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
        Self::k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
