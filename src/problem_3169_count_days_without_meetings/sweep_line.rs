pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let days = days.cast_unsigned();

        let mut meetings = meetings
            .into_iter()
            .flat_map(|meeting| {
                let [start, end] = <[_; 2]>::map(meeting.try_into().ok().unwrap(), i32::cast_unsigned);

                [start << 2, (end << 2) | 2]
            })
            .collect::<Box<_>>();

        meetings.sort_unstable();

        let mut result = 0;
        let mut start = 1;
        let mut count = 0;

        for event in meetings {
            count = count + 1 - (event & 3);

            if count == 0 {
                start = (event >> 2) + 1;
            } else if start != 0 {
                result += (event >> 2) - start;
                start = 0;
            }
        }

        result += days + 1 - start;

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        Self::count_days(days, meetings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
