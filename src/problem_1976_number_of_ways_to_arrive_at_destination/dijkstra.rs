pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node {
    distance: u64,
    node: u8,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as u8;
        let mut graph = vec![Vec::new(); usize::from(n)].into_boxed_slice();
        let mut states = vec![(u64::MAX, 0_u32); usize::from(n)].into_boxed_slice();

        for road in roads {
            let [from, to, time] = road.try_into().ok().unwrap();
            let from = from as u8 as usize;
            let to = to as u8 as usize;
            let time = time as u32;

            graph[from].push((to as u8, time));
            graph[to].push((from as u8, time));
        }

        states[usize::from(n) - 1] = (0, 1);

        let mut queue = BinaryHeap::new();

        let mut node = Node {
            distance: 0,
            node: n - 1,
        };

        let mut node_ways = 1;

        while node.node != 0 {
            for &(neighbor, time) in &graph[usize::from(node.node)] {
                let candidate_distance = node.distance + u64::from(time);
                let state = &mut states[usize::from(neighbor)];

                match candidate_distance.cmp(&state.0) {
                    Ordering::Less => {
                        *state = (candidate_distance, node_ways);

                        queue.push(Node {
                            distance: candidate_distance,
                            node: neighbor,
                        });
                    }
                    Ordering::Equal => {
                        let ways = state.1 + node_ways;

                        state.1 = ways.checked_sub(1_000_000_007).unwrap_or(ways);
                    }
                    Ordering::Greater => {}
                }
            }

            loop {
                let next = queue.pop().unwrap();
                let state = states[usize::from(next.node)];

                if next.distance == state.0 {
                    node = next;
                    node_ways = state.1;

                    break;
                }
            }
        }

        states.first().unwrap().1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        Self::count_paths(n, roads)
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn test_node_partial_eq() {
        assert!(Node { distance: 2, node: 3 } == Node { distance: 2, node: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
