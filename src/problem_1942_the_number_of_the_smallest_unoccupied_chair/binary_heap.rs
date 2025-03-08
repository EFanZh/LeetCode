pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::mem;

struct Item {
    leaving: u32,
    chair: u32,
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
        other.leaving.cmp(&self.leaving)
    }
}

struct Allocator {
    occupied: BinaryHeap<Item>,
    idle: BinaryHeap<Reverse<u32>>,
    prev_idle: u32,
}

impl Allocator {
    fn allocate(&mut self, arrival: u32) -> u32 {
        while let Some(&Item { leaving, chair }) = self.occupied.peek() {
            if leaving <= arrival {
                self.occupied.pop();

                if chair == self.prev_idle {
                    self.prev_idle = self.prev_idle.wrapping_sub(1);
                } else {
                    self.idle.push(Reverse(chair));
                }
            } else {
                break;
            }
        }

        if let Some(Reverse(chair)) = self.idle.pop() {
            chair
        } else {
            self.prev_idle = self.prev_idle.wrapping_add(1);

            self.prev_idle
        }
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Self {
            occupied: BinaryHeap::new(),
            idle: BinaryHeap::new(),
            prev_idle: u32::MAX,
        }
    }
}

impl Solution {
    fn partition<T>(values: &mut [T], mut f: impl FnMut(&T) -> bool) -> usize {
        let mut result = 0;
        let mut iter = values.iter_mut();

        'outer: while let Some(left) = iter.next() {
            if !f(left) {
                loop {
                    if let Some(right) = iter.next_back() {
                        if f(right) {
                            mem::swap(left, right);

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }

            result += 1;
        }

        result
    }

    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut times = times
            .into_iter()
            .map(|time| {
                let [arrival, leaving] = time.try_into().ok().unwrap();

                (arrival as u32, leaving as u32)
            })
            .collect::<Box<_>>();

        let target_friend = target_friend as u32 as usize;
        let target_arrival = times[target_friend].0;
        let split = Self::partition(&mut times, |&(arrival_time, _)| arrival_time < target_arrival);
        let times = &mut times[..split];

        times.sort_unstable_by_key(|&(arrival_time, _)| arrival_time);

        let mut allocator = Allocator::default();

        for &(arrival, leaving) in &*times {
            let chair = allocator.allocate(arrival);

            allocator.occupied.push(Item { leaving, chair });
        }

        allocator.allocate(target_arrival) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        Self::smallest_chair(times, target_friend)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { leaving: 2, chair: 3 } == Item { leaving: 2, chair: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
