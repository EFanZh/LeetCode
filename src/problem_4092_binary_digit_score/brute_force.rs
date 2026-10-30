pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn calculate_score(a: i32, b: i32) -> i32 {
        let s1 = a.count_ones();
        let s2 = b.count_ones();

        (if s1 < s2 { s2 / s1 } else { s2 * s1 }).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate_score(a: i32, b: i32) -> i32 {
        Self::calculate_score(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
