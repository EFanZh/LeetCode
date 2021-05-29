pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        matches!(num, 6 | 28 | 496 | 8128 | 33_550_336)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_perfect_number(num: i32) -> bool {
        Self::check_perfect_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
