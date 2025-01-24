pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(s: String) -> u16 {
        const MONTH_START_DAYS: [u16; 11] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304];

        let [m0, m1, _, d0, d1]: [_; 5] = s.into_bytes().try_into().ok().unwrap();
        let month = 10 * (m0 - b'0') + (m1 - b'0') - 1;
        let day = 10 * (d0 - b'0') + (d1 - b'0') - 1;

        MONTH_START_DAYS.get(usize::from(month)).copied().unwrap_or(334) + u16::from(day)
    }

    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let arrive_alice = Self::parse(arrive_alice);
        let leave_alice = Self::parse(leave_alice);
        let arrive_bob = Self::parse(arrive_bob);
        let leave_bob = Self::parse(leave_bob);

        i32::from((leave_alice.min(leave_bob) + 1).saturating_sub(arrive_alice.max(arrive_bob)))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        Self::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
