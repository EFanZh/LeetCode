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
                let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
                let from = from as u32;
                let to = to as u32;

                graph[from as usize].push(to);
                graph[to as usize].push(from);
            }

            let mut stack = Vec::new();
            let mut iter = mem::take(&mut graph[source as u32 as usize]).into_iter();

            loop {
                if let Some(node) = iter.next() {
                    if node == destination {
                        return true;
                    }

                    stack.push(iter);

                    iter = mem::take(&mut graph[node as usize]).into_iter();
                } else if let Some(next_iter) = stack.pop() {
                    iter = next_iter;
                } else {
                    return false;
                }
            }
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
