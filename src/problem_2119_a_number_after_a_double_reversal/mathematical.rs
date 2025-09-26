pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || !(num as u32).is_multiple_of(10)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_same_after_reversals(num: i32) -> bool {
        Self::is_same_after_reversals(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
