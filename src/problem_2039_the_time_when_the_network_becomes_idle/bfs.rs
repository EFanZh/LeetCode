pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;
use std::num::NonZeroU32;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut graph = vec![Vec::new(); patience.len()].into_boxed_slice();

        for edge in edges {
            let [from, to] = edge.try_into().ok().unwrap();
            let from = from as u32;
            let to = to as u32;

            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut states = patience;
        let mut queue = VecDeque::from([0_u32]);
        let mut double_distance = 0_u32;
        let mut distance_minus_1 = u32::MAX;
        let mut result = 0_u32;

        loop {
            double_distance += 4;
            distance_minus_1 = distance_minus_1.wrapping_add(2);

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for &neighbor in &graph[node as usize] {
                    if let Some(patience) = NonZeroU32::new(mem::take(&mut states[neighbor as usize]) as _) {
                        result = result.max(double_distance - distance_minus_1 % patience);

                        queue.push_back(neighbor);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        Self::network_becomes_idle(edges, patience)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
