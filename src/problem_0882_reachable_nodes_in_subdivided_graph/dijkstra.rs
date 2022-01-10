pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryInto;

struct Item {
    node: u32,
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
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let max_moves = max_moves as u32;
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];

        for edge in edges {
            let [from, to, count]: [_; 3] = edge.as_slice().try_into().unwrap();
            let from = from as u32;
            let to = to as u32;
            let count = count as u32;

            graph[from as usize].push((to, count));
            graph[to as usize].push((from, count));
        }

        let mut result = 1;
        let mut edge_covered = vec![0; (n - 1) * n / 2];
        let mut distances = vec![u32::MAX; n];
        let mut queue = BinaryHeap::new();

        distances[0] = 0;

        let mut item = Item { node: 0, distance: 0 };

        loop {
            for &(next_node, count) in &graph[item.node as usize] {
                let new_distance = item.distance + count + 1;

                // Update distance.

                if new_distance <= max_moves {
                    let distance = &mut distances[next_node as usize];

                    if new_distance < *distance {
                        *distance = new_distance;

                        queue.push(Item {
                            node: next_node,
                            distance: new_distance,
                        });
                    }
                }

                // Check coverage.

                let (from, to) = if next_node < item.node {
                    (next_node, item.node)
                } else {
                    (item.node, next_node)
                };

                let covered = &mut edge_covered[((to - 1) * to / 2 + from) as usize];
                let extra_length = max_moves - item.distance;

                if *covered == 0 {
                    *covered = extra_length.min(count);
                    result += *covered;
                } else {
                    result += extra_length.min(count - *covered);
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.distance == distances[next_item.node as usize] {
                        item = next_item;
                        result += 1;

                        break;
                    }
                } else {
                    return result as _;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        Self::reachable_nodes(edges, max_moves, n)
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
