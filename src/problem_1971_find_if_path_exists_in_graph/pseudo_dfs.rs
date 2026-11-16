pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            true
        } else {
            let destination = destination.cast_unsigned();
            let mut graph = vec![Vec::new(); n.cast_unsigned() as usize].into_boxed_slice();

            for edge in edges {
                let [from, to] = edge.try_into().ok().unwrap();
                let from = from.cast_unsigned();
                let to = to.cast_unsigned();

                graph[from as usize].push(to);
                graph[to as usize].push(from);
            }

            let mut stack = Vec::new();
            let mut neighbors = mem::take(&mut graph[source.cast_unsigned() as usize]);

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
