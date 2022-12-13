pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::mem;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let distance_threshold = distance_threshold as u32;

        // Floyd–Warshall algorithm.

        let mut buffer = vec![u32::MAX; n * n * 2];
        let (mut cache, mut temp) = buffer.split_at_mut(n * n);

        for edge in edges {
            let [from, to, weight]: [_; 3] = edge.try_into().ok().unwrap();
            let from = from as usize;
            let to = to as usize;
            let weight = weight as _;

            cache[n * from + to] = weight;
            cache[n * to + from] = weight;
        }

        for distance in cache.iter_mut().step_by(n + 1) {
            *distance = 0;
        }

        for node in 0..n {
            let node_to_ends = &cache[n * node..n * node + n];
            let mut iter = temp.iter_mut().zip(cache.iter().copied());

            for &start_to_node in cache[node..].iter().step_by(n) {
                for &node_to_end in node_to_ends {
                    let (target, prev) = iter.next().unwrap();

                    *target = prev.min(start_to_node.saturating_add(node_to_end));
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        // Search.

        let mut candidate = n;
        let mut candidate_neighbors = usize::MAX;

        for (i, row) in cache.chunks_exact(n).enumerate().rev() {
            let count = row.iter().filter(|&&distance| distance <= distance_threshold).count();

            if count < candidate_neighbors {
                candidate_neighbors = count;
                candidate = i;
            }
        }

        candidate as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        Self::find_the_city(n, edges, distance_threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
