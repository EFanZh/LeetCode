pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let doubled_result = (60 * hour - 11 * minutes).abs();

        f64::from(doubled_result.min(720 - doubled_result)) / 2.0
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn angle_clock(hour: i32, minutes: i32) -> f64 {
        Self::angle_clock(hour, minutes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
