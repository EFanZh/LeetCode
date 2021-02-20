pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut intervals = [(0, 0); 26];

        for (i, c) in (1..).zip(s.bytes()) {
            let (start, end) = &mut intervals[usize::from(c - b'a')];

            if *start == 0 {
                *start = i;
            }

            *end = i;
        }

        intervals.sort_unstable_by_key(|&(start, _)| start);

        let i = intervals.iter().position(|&(start, _)| start != 0).unwrap();
        let mut result = Vec::with_capacity(intervals.len() - i);
        let (mut prev_start, mut prev_end) = intervals[i];

        for &(start, end) in &intervals[i + 1..] {
            if start < prev_end {
                prev_end = prev_end.max(end);
            } else {
                result.push(prev_end - prev_start + 1);

                prev_start = start;
                prev_end = end;
            }
        }

        result.push(prev_end - prev_start + 1);

        result
    }
}

impl super::Solution for Solution {
    fn partition_labels(s: String) -> Vec<i32> {
        Self::partition_labels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
