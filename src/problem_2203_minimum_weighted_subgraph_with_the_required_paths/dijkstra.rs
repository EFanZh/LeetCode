pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;

struct Item {
    distance: u64,
    node: usize,
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
    fn dijkstra(graph: &[Vec<(u32, u32)>], start: u32, distances: &mut [u64], queue: &mut BinaryHeap<Item>) {
        let start = start as usize;

        distances[start] = 0;

        let mut item = Item {
            distance: 0,
            node: start,
        };

        'outer: loop {
            for &(neighbor, weight) in &graph[item.node] {
                let neighbor = neighbor as usize;
                let candidate_distance = item.distance + u64::from(weight);
                let distance = &mut distances[neighbor];

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
                    if next_item.distance == distances[next_item.node] {
                        item = next_item;

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }

    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as u32 as usize;

        // Build forward graph.

        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in &edges {
            let [from, to, weight] = <[_; 3]>::map(edge.as_slice().try_into().ok().unwrap(), |x| x as u32);

            graph[from as usize].push((to, weight));
        }

        // Calculate forward distances.

        let mut distances = vec![u64::MAX; n * 2].into_boxed_slice();
        let (distances_1, distances_2) = distances.split_at_mut(n);
        let mut queue = BinaryHeap::new();

        Self::dijkstra(&graph, src1 as _, distances_1, &mut queue);
        Self::dijkstra(&graph, src2 as _, distances_2, &mut queue);

        distances_1
            .iter_mut()
            .zip(&mut *distances_2)
            .for_each(|(target, source)| *target = target.saturating_add(mem::replace(source, u64::MAX)));

        // Build backward graph.

        graph.iter_mut().for_each(Vec::clear);

        for edge in &edges {
            let [from, to, weight] = <[_; 3]>::map(edge.as_slice().try_into().ok().unwrap(), |x| x as u32);

            graph[to as usize].push((from, weight));
        }

        drop(edges);

        // Calculate backward distances.

        Self::dijkstra(&graph, dest as _, distances_2, &mut queue);

        distances_1
            .iter()
            .zip(&*distances_2)
            .fold(u64::MAX, |distance, (&x, &y)| distance.min(x.saturating_add(y))) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        Self::minimum_weight(n, edges, src1, src2, dest)
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
