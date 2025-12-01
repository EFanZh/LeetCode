pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut counts = [0; 24];
        let mut result = 0;

        for hour in hours {
            let hour = (hour.cast_unsigned() % 24) as usize;

            result += counts[if hour == 0 { 0 } else { 24 - hour }];
            counts[hour] += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        Self::count_complete_day_pairs(hours)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
