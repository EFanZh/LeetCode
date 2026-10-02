pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .into_iter()
            .flat_map(|interval| {
                let [start, end] = interval.try_into().ok().unwrap();

                [start * 2, end * 2 + 1]
            })
            .collect::<Box<_>>();

        intervals.sort_unstable();

        let mut count = 0;
        let mut result = 0;

        for state in intervals {
            if state & 1 == 0 {
                result += count;
                count += 1;
            } else {
                count -= 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        Self::count_intersecting_intervals(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
