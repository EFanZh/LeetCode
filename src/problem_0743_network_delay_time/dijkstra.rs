pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryInto;

struct Item {
    node: usize,
    distance: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.distance, &self.distance)
    }
}

impl Solution {
    fn relax(queue: &mut BinaryHeap<Item>, distances: &mut [u32], node: usize, distance: u32) {
        let current_distance = &mut distances[node];

        if distance < *current_distance {
            *current_distance = distance;

            queue.push(Item { node, distance });
        }
    }

    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as _;
        let mut graph = vec![Vec::new(); n];

        for edge in times {
            let [from, to, weight]: [i32; 3] = edge.as_slice().try_into().unwrap();

            graph[(from - 1) as usize].push(((to - 1) as usize, weight as u32));
        }

        let mut item = Item {
            node: (k - 1) as _,
            distance: 0,
        };

        let mut queue = BinaryHeap::new();
        let mut distances = vec![u32::MAX; n];

        distances[item.node] = 0;

        'outer: loop {
            for &(next_node, weight) in &graph[item.node] {
                Self::relax(&mut queue, &mut distances, next_node, item.distance + weight);
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.distance == distances[next_item.node] {
                        item = next_item;

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        distances.iter().copied().max().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        Self::network_delay_time(times, n, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { node: 3, distance: 2 } == Item { node: 4, distance: 2 });
        assert!(Item { node: 3, distance: 2 } != Item { node: 3, distance: 4 });
    }
}
