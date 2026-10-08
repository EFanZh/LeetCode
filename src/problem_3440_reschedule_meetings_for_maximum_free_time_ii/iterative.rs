pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let event_time = event_time.cast_unsigned();

        let mut meetings = start_time
            .into_iter()
            .zip(end_time)
            .map(|(start_time, end_time)| (start_time.cast_unsigned(), end_time.cast_unsigned()))
            .collect::<Box<_>>();

        meetings.sort_unstable_by_key(|&(start_time, _)| start_time);

        let mut max_gap = 0;
        let mut right_bound = event_time;

        let right_max_gaps = meetings
            .iter()
            .skip(1)
            .rev()
            .map(|&(start_time, end_time)| {
                let gap = right_bound - end_time;

                max_gap = max_gap.max(gap);
                right_bound = start_time;

                max_gap
            })
            .collect::<Box<_>>();

        let mut meetings_iter = meetings.into_iter();
        let mut prev_meeting = meetings_iter.next().unwrap();
        let mut prev_gap = prev_meeting.0;
        let mut left_max_gap = 0;
        let mut result = 0;

        meetings_iter
            .zip(right_max_gaps.into_iter().rev())
            .for_each(|(meeting, right_max_gap)| {
                let duration = prev_meeting.1 - prev_meeting.0;
                let gap = meeting.0 - prev_meeting.1;
                let mut free_time = prev_gap + gap;

                if left_max_gap >= duration || right_max_gap >= duration {
                    free_time += duration;
                }

                result = result.max(free_time);
                left_max_gap = left_max_gap.max(prev_gap);
                prev_gap = gap;
                prev_meeting = meeting;
            });

        let duration = prev_meeting.1 - prev_meeting.0;
        let gap = event_time - prev_meeting.1;
        let mut free_time = prev_gap + gap;

        if left_max_gap >= duration {
            free_time += duration;
        }

        result.max(free_time).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        Self::max_free_time(event_time, start_time, end_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
