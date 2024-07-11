pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    distance: u32,
    node: u32,
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
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;
        let mut states = vec![(u32::MAX, 0_u32); n].into_boxed_slice();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to, weight]: [_; 3] = edge.try_into().ok().unwrap();
            let from = from as u32 - 1;
            let to = to as u32 - 1;
            let weight = weight as u32;

            graph[from as usize].push((to, weight));
            graph[to as usize].push((from, weight));
        }

        *states.last_mut().unwrap() = (0, 1);

        let mut queue = BinaryHeap::new();

        let mut item = Item {
            distance: 0,
            node: n as u32 - 1,
        };

        'outer: loop {
            let node_paths = states[item.node as usize].1;

            for &(neighbor, weight) in &graph[item.node as usize] {
                let (neighbor_distance, neighbor_paths) = &mut states[neighbor as usize];
                let candidate_distance = item.distance + weight;

                if candidate_distance < *neighbor_distance {
                    *neighbor_distance = candidate_distance;

                    queue.push(Item {
                        distance: candidate_distance,
                        node: neighbor,
                    });
                }

                if item.distance < *neighbor_distance {
                    *neighbor_paths += node_paths;
                    *neighbor_paths = neighbor_paths.checked_sub(1_000_000_007).unwrap_or(*neighbor_paths);
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.distance == states[next_item.node as usize].0 {
                        item = next_item;

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        states[0].1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::count_restricted_paths(n, edges)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { distance: 2, node: 3 } == Item { distance: 2, node: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
