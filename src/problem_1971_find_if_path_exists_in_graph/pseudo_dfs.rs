pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            true
        } else {
            let destination = destination as u32;
            let mut graph = vec![Vec::new(); n as u32 as usize].into_boxed_slice();

            for edge in edges {
                let [from, to] = edge.try_into().ok().unwrap();
                let from = from as u32;
                let to = to as u32;

                graph[from as usize].push(to);
                graph[to as usize].push(from);
            }

            let mut stack = Vec::new();
            let mut neighbors = mem::take(&mut graph[source as u32 as usize]);

            loop {
                for neighbor in neighbors {
                    if neighbor == destination {
                        return true;
                    }

                    let next_neighbors = mem::take(&mut graph[neighbor as usize]);

                    if !next_neighbors.is_empty() {
                        stack.push(next_neighbors);
                    }
                }

                if let Some(next_node) = stack.pop() {
                    neighbors = next_node;
                } else {
                    break;
                }
            }

            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        Self::valid_path(n, edges, source, destination)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
