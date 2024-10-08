pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(non_snake_case, reason = "required")]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn hamming_weight(n: u32) -> i32 {
        Self::hammingWeight(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
