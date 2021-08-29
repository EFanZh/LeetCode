pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = target.wrapping_abs() as u32;
        let mut low = ((f64::from(target * 2) + 0.25).sqrt() - 0.5).ceil() as u32;
        let reached = low * (low + 1) / 2;

        if reached % 2 != target % 2 {
            low = (low + 1) | 1;
        }

        low as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reach_number(target: i32) -> i32 {
        Self::reach_number(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
