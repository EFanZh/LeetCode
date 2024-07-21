pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ptr;

struct Node<'a> {
    distance: u64,
    state: &'a (Vec<(u8, u32)>, Cell<u64>, Cell<u32>),
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Node<'_> {}

impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as u8;
        let mut nodes = vec![(Vec::new(), Cell::new(u64::MAX), Cell::new(0_u32)); usize::from(n)].into_boxed_slice();

        for road in roads {
            let [from, to, time]: [_; 3] = road.try_into().ok().unwrap();
            let from = from as u8 as usize;
            let to = to as u8 as usize;
            let time = time as u32;

            nodes[from].0.push((to as u8, time));
            nodes[to].0.push((from as u8, time));
        }

        let last = nodes.last().unwrap();

        last.1.set(0);
        last.2.set(1);

        let first = nodes.first().unwrap();
        let mut queue = BinaryHeap::new();

        let mut node = Node {
            distance: 0,
            state: last,
        };

        while !ptr::eq(node.state, first) {
            let node_ways = node.state.2.get();

            for &(neighbor, time) in &node.state.0 {
                let candidate_distance = node.distance + u64::from(time);
                let neighbor_state = &nodes[usize::from(neighbor)];

                match candidate_distance.cmp(&neighbor_state.1.get()) {
                    Ordering::Less => {
                        neighbor_state.1.set(candidate_distance);
                        neighbor_state.2.set(node_ways);

                        queue.push(Node {
                            distance: candidate_distance,
                            state: neighbor_state,
                        });
                    }
                    Ordering::Equal => {
                        let ways = neighbor_state.2.get() + node_ways;

                        neighbor_state.2.set(ways.checked_sub(1_000_000_007).unwrap_or(ways));
                    }
                    Ordering::Greater => {}
                }
            }

            loop {
                let next = queue.pop().unwrap();

                if next.distance == next.state.1.get() {
                    node = next;

                    break;
                }
            }
        }

        nodes.first().unwrap().2.get() as _
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
    use std::cell::Cell;

    #[test]
    fn test_node_partial_eq() {
        assert!(
            Node {
                distance: 2,
                state: &(vec![(3, 5)], Cell::new(7), Cell::new(11)),
            } == Node {
                distance: 2,
                state: &(vec![(13, 17), (23, 29)], Cell::new(31), Cell::new(37)),
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
