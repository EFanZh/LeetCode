pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mask = (1_i32 << (32 - num.leading_zeros())).wrapping_sub(1);

        !num & mask
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_complement(num: i32) -> i32 {
        Self::find_complement(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
