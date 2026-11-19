pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::iter;

impl Solution {
    fn bfs_farthest(graph: &[Vec<u32>], node: u32, parent: u32, queue: &mut VecDeque<(u32, u32)>) -> (u32, u32) {
        let mut farthest = node;
        let mut distance = 0;

        queue.push_back((node, parent));

        loop {
            for _ in 0..queue.len() {
                let (node, parent) = queue.pop_front().unwrap();

                for &child in &graph[node as usize] {
                    if child != parent {
                        queue.push_back((child, node));
                    }
                }
            }

            if let Some(front) = queue.front() {
                farthest = front.0;
                distance += 1;
            } else {
                break;
            }
        }

        (farthest, distance)
    }

    fn diameter(edges: Vec<Vec<i32>>, graph: &mut [Vec<u32>], queue: &mut VecDeque<(u32, u32)>) -> u32 {
        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let farthest = Self::bfs_farthest(graph, 0, u32::MAX, queue).0;

        Self::bfs_farthest(graph, farthest, u32::MAX, queue).1
    }

    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let (edges1, edges2) = if edges2.len() < edges1.len() {
            (edges2, edges1)
        } else {
            (edges1, edges2)
        };

        let mut graph = iter::repeat_with(|| Vec::with_capacity(1))
            .take(edges2.len() + 1)
            .collect::<Box<_>>();

        let mut queue = VecDeque::new();
        let n1 = edges1.len() + 1;
        let diameter_1 = Self::diameter(edges1, &mut graph, &mut queue);

        graph[..n1].iter_mut().for_each(Vec::clear);

        let diameter_2 = Self::diameter(edges2, &mut graph, &mut queue);

        diameter_1
            .max(diameter_2)
            .max(diameter_1.div_ceil(2) + diameter_2.div_ceil(2) + 1)
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        Self::minimum_diameter_after_merge(edges1, edges2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
