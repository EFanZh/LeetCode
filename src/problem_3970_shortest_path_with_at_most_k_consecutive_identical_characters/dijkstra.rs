pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    distance: u32,
    node: (u16, u8),
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
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n.cast_unsigned() as usize;
        let labels = labels.as_bytes();
        let k = k.cast_unsigned() as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to, weight] = <[_; 3]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[from as usize].push((to, weight));
        }

        let target = n as u16 - 1;
        let mut distances = vec![u32::MAX; k * n].into_boxed_slice();

        distances[0] = 0;

        let mut queue = BinaryHeap::new();

        let mut item = Item {
            distance: 0,
            node: (0, 0),
        };

        loop {
            if item.node.0 == target {
                return item.distance.cast_signed();
            }

            let label = labels[item.node.0 as usize];

            for &(neighbor, weight) in &graph[item.node.0 as usize] {
                let neighbor = neighbor as usize;
                let neighbor_label = labels[neighbor];

                if neighbor_label != label || item.node.1 + 1 < k as u8 {
                    let candidate_distance = item.distance + weight;
                    let next_length = if neighbor_label == label { item.node.1 + 1 } else { 0 };
                    let distance = &mut distances[k * neighbor + usize::from(next_length)];

                    if candidate_distance < *distance {
                        *distance = candidate_distance;

                        queue.push(Item {
                            distance: candidate_distance,
                            node: (neighbor as u16, next_length),
                        });
                    }
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.distance
                        == distances[k * usize::from(next_item.node.0) + usize::from(next_item.node.1)]
                    {
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
    fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        Self::shortest_path(n, edges, labels, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
