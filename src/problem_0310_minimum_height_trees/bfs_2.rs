pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryInto;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as _;
        let mut graph = vec![Vec::with_capacity(1); n];
        let mut indegrees = vec![0; n];

        for edge in edges {
            let [from, to]: [i32; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
            graph[to as usize].push(from);
            indegrees[from as usize] += 1;
            indegrees[to as usize] += 1;
        }

        let mut queue = indegrees
            .iter()
            .enumerate()
            .filter_map(|(i, &num)| if num < 2 { Some(i as _) } else { None })
            .collect::<VecDeque<_>>();

        let mut internal_nodes = n - queue.len();

        while internal_nodes != 0 {
            for _ in 0..queue.len() {
                for &next in &graph[queue.pop_front().unwrap() as usize] {
                    let indegree = &mut indegrees[next as usize];

                    *indegree -= 1;

                    if *indegree == 1 {
                        queue.push_back(next);
                    }
                }
            }

            internal_nodes -= queue.len();
        }

        queue.into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_min_height_trees(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
