pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events
            .into_iter()
            .map(|event| {
                let [start, end]: [_; 2] = event.try_into().ok().unwrap();

                (start as u32, end as u32)
            })
            .collect::<Vec<_>>();

        events.sort_unstable();

        let mut result = 0;
        let mut time = 0;
        let mut queue = BinaryHeap::<Reverse<_>>::new();

        for (start, end) in events {
            while time < start {
                if let Some(Reverse(top)) = queue.pop() {
                    if time <= top {
                        result += 1;
                        time += 1;
                    }
                } else {
                    time = start;

                    break;
                }
            }

            queue.push(Reverse(end));
        }

        while let Some(Reverse(end)) = queue.pop() {
            if time <= end {
                result += 1;
                time += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_events(events: Vec<Vec<i32>>) -> i32 {
        Self::max_events(events)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
