pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let buckets = buckets as u32;
        let minutes_to_die = minutes_to_die as u32;
        let minutes_to_test = minutes_to_test as u32;
        let base = minutes_to_test / minutes_to_die + 1;

        f64::from(buckets).log(f64::from(base)).ceil() as _
    }
}

impl super::Solution for Solution {
    fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        Self::poor_pigs(buckets, minutes_to_die, minutes_to_test)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
