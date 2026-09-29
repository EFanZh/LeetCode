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
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n.cast_unsigned() as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to, weight] = <[_; 3]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[from as usize].push((to, weight));
            graph[to as usize].push((from, weight * 2));
        }

        let target = n as u32 - 1;
        let mut distances = vec![u32::MAX; n].into_boxed_slice();

        distances[0] = 0;

        let mut queue = BinaryHeap::new();
        let mut item = Item { distance: 0, node: 0 };

        loop {
            if item.node == target {
                return item.distance.cast_signed();
            }

            for &(neighbor, weight) in &graph[item.node as usize] {
                let candidate_distance = item.distance + weight;
                let distance = &mut distances[neighbor as usize];

                if candidate_distance < *distance {
                    *distance = candidate_distance;

                    queue.push(Item {
                        distance: candidate_distance,
                        node: neighbor,
                    });
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.distance == distances[next_item.node as usize] {
                        item = next_item;

                        break;
                    }
                } else {
                    return -1;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::min_cost(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
