pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut events = Vec::with_capacity(n * 2);

        for (i, (start, end)) in (0_u16..).zip(start_time.into_iter().zip(end_time)) {
            events.extend([(start as u32 * 2 + 1, i), (end as u32 * 2, i)]);
        }

        events.sort_unstable_by_key(|&(key, _)| key);

        let mut cache = vec![0_u32; n];
        let mut max_profit = 0;

        for (position, i) in events {
            let i = usize::from(i);

            if position % 2 == 0 {
                max_profit = max_profit.max(cache[i]);
            } else {
                cache[i] = max_profit + profit[i] as u32;
            }
        }

        max_profit as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        Self::job_scheduling(start_time, end_time, profit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
