pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series.get(1..).map_or(0, |rest| {
            time_series
                .iter()
                .zip(rest)
                .map(|(left, right)| (right - left).min(duration))
                .sum::<i32>()
                + duration
        })
    }
}

impl super::Solution for Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        Self::find_poisoned_duration(time_series, duration)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
