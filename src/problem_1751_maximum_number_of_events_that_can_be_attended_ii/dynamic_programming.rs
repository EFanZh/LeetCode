pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::convert::TryInto;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events
            .into_iter()
            .map(|event| {
                let [start, end, value]: [_; 3] = event.try_into().ok().unwrap();

                (end as u32, start as u32, value as u32, Cell::new(0_u32))
            })
            .collect::<Box<_>>();

        let k = k as u32 as usize;

        events.sort_unstable();

        for (i, event) in events.iter().enumerate() {
            event.3.set(
                events[..i]
                    .partition_point(|prev_event| prev_event.0 < event.1)
                    .wrapping_sub(1) as _,
            );
        }

        let mut cache = vec![0; k * events.len()].into_boxed_slice();

        for (i, event) in events.iter().enumerate() {
            let (prev_rows, row) = cache[..k * (i + 1)].split_at_mut(k * i);

            for (j, value) in row.iter_mut().enumerate() {
                let value_if_drop = prev_rows
                    .get(k.wrapping_mul(i.wrapping_sub(1)).wrapping_add(j))
                    .copied()
                    .unwrap_or(0);

                let value_if_choose = if j == 0 {
                    event.2
                } else {
                    prev_rows
                        .get(k.wrapping_mul(event.3.get() as usize).wrapping_add(j - 1))
                        .copied()
                        .unwrap_or(0)
                        + event.2
                };

                *value = value_if_drop.max(value_if_choose);
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::max_value(events, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
