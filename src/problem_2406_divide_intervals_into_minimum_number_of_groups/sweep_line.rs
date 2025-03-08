pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .into_iter()
            .flat_map(|interval| {
                let [left, right] = interval.try_into().ok().unwrap();

                [(left as u32) << 2, ((right as u32) << 2) | 2]
            })
            .collect::<Box<_>>();

        intervals.sort_unstable();

        let mut max_overlapped = 0;
        let mut overlapped = 0;

        for position in &*intervals {
            overlapped += 1;
            overlapped -= position & 2;
            max_overlapped = u32::max(max_overlapped, overlapped);
        }

        max_overlapped as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        Self::min_groups(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
