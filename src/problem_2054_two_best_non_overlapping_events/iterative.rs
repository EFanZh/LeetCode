pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn parse_event(event: &[i32]) -> &[i32; 3] {
        event.try_into().ok().unwrap()
    }

    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        const PROBE: u64 = 1 << 32;

        let mut events_2 = Vec::with_capacity(events.len() * 2);

        events_2.extend(events.iter().map(|event| {
            let &[start, _, value] = Self::parse_event(event);

            (u64::from(start as u32) << 33) | u64::from(value as u32)
        }));

        events_2.extend(events.iter().map(|event| {
            let &[_, end, value] = Self::parse_event(event);

            (u64::from(end as u32) << 33) | PROBE | u64::from(value as u32)
        }));

        drop(events);

        events_2.sort_unstable();

        let mut result = 0;
        let mut max = 0;

        for state in events_2 {
            if state & PROBE == 0 {
                result = result.max(max + state as u32);
            } else {
                max = max.max(state as u32);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        Self::max_two_events(events)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
