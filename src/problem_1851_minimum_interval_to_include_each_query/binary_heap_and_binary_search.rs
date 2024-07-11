pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    size: u32,
    index: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.size.cmp(&self.size)
    }
}

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = intervals.len();
        let mut new_intervals = Vec::with_capacity(n * 2);

        for (i, interval) in (0_u32..).zip(intervals) {
            let [left, right]: [_; 2] = interval.try_into().ok().unwrap();
            let (left, right) = (left as u32, right as u32 + 1);

            new_intervals.extend([(left, right - left, i), (right, 0, i)]);
        }

        new_intervals.sort_unstable_by_key(|&(x, size, _)| (x, size));

        let mut queue = BinaryHeap::new();
        let mut in_queue = vec![false; n];
        let mut splits = Vec::with_capacity(n);
        let mut prev_size = 0;

        for (x, size, i) in new_intervals {
            let is_start = size != 0;

            if is_start {
                queue.push(Item { size, index: i });
            }

            in_queue[i as usize] = is_start;

            let size = loop {
                if let Some(&Item {
                    size: top_size,
                    index: top_index,
                }) = queue.peek()
                {
                    if in_queue[top_index as usize] {
                        break top_size;
                    }

                    queue.pop();
                } else {
                    break u32::MAX;
                }
            };

            if size != prev_size {
                splits.push((x, size));

                prev_size = size;
            }
        }

        let mut queries = queries;

        for target in &mut queries {
            let query = *target as u32;
            let i = splits.partition_point(|&(x, _)| x <= query);

            *target = splits.get(i.wrapping_sub(1)).map_or(u32::MAX, |&(_, i)| i) as _;
        }

        queries
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        Self::min_interval(intervals, queries)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { size: 2, index: 3 } == Item { size: 2, index: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
