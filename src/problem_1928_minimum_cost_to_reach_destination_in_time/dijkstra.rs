pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    cost: u32,
    countdown: u16,
    node: u16,
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
        other.cost.cmp(&self.cost)
    }
}

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let max_time = max_time as u16;
        let n = passing_fees.len();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to, time] = <[_; 3]>::map(edge.try_into().ok().unwrap(), |x| x as u16);

            graph[usize::from(from)].push((to, time));
            graph[usize::from(to)].push((from, time));
        }

        let mut queue = BinaryHeap::new();
        let mut countdowns = vec![0; n].into_boxed_slice();

        let mut item = Item {
            cost: passing_fees[n - 1] as _,
            countdown: max_time,
            node: n as u16 - 1,
        };

        loop {
            let countdown = &mut countdowns[usize::from(item.node)];

            if *countdown < item.countdown {
                *countdown = item.countdown;

                for &(neighbor, time) in &graph[usize::from(item.node)] {
                    if let Some(candidate_countdown) = item.countdown.checked_sub(time) {
                        queue.push(Item {
                            cost: item.cost + passing_fees[usize::from(neighbor)] as u32,
                            countdown: candidate_countdown,
                            node: neighbor,
                        });
                    }
                }
            }

            if let Some(next) = queue.pop() {
                if next.node == 0 {
                    return next.cost as _;
                }

                item = next;
            } else {
                return -1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        Self::min_cost(max_time, edges, passing_fees)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                cost: 2,
                countdown: 5,
                node: 7,
            } == Item {
                cost: 2,
                countdown: 11,
                node: 13,
            }
        );

        assert!(
            Item {
                cost: 2,
                countdown: 5,
                node: 7,
            } != Item {
                cost: 3,
                countdown: 5,
                node: 7,
            }
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
