pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::with_capacity(1); n as _];
        let mut indegrees = vec![0; n as _];

        for edge in edges {
            let [from, to] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
            graph[to as usize].push(from);
            indegrees[from as usize] += 1;
            indegrees[to as usize] += 1;
        }

        let mut queue = indegrees
            .iter()
            .enumerate()
            .filter(|&(_, &num)| num < 2)
            .map(|(i, _)| i as _)
            .collect::<Vec<_>>();

        let mut temp = Vec::new();

        loop {
            for &node in &queue {
                for &next in &graph[node as usize] {
                    let indegree = &mut indegrees[next as usize];

                    *indegree -= 1;

                    if *indegree == 1 {
                        temp.push(next);
                    }
                }
            }

            if temp.is_empty() {
                break;
            }

            queue.clear();

            mem::swap(&mut queue, &mut temp);
        }

        queue
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
