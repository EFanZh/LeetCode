pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as u32 as usize;
        let mut visited = vec![false; n].into_boxed_slice();

        for x in restricted {
            visited[x as u32 as usize] = true;
        }

        let mut neighbors = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), |x| x as u32);
            let (from_usize, to_usize) = (from as usize, to as usize);

            if !(visited[from_usize] || visited[to_usize]) {
                neighbors[from_usize].push(to);
                neighbors[to_usize].push(from);
            }
        }

        let mut result = 1;
        let mut queue = VecDeque::new();
        let mut node = 0;

        visited[0] = true;

        loop {
            for &neighbor in &neighbors[node as usize] {
                if let state @ false = &mut visited[neighbor as usize] {
                    *state = true;
                    result += 1;

                    queue.push_back(neighbor);
                }
            }

            if let Some(next_node) = queue.pop_front() {
                node = next_node;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        Self::reachable_nodes(n, edges, restricted)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
