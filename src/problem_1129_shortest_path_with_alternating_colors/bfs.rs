pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryFrom;

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n * 2];

        for edge in &red_edges {
            let [from, to] = <[_; 2]>::try_from(edge.as_slice()).unwrap();

            graph[from as usize * 2 + 1].push(to as usize * 2);
        }

        drop(red_edges);

        for edge in &blue_edges {
            let [from, to] = <[_; 2]>::try_from(edge.as_slice()).unwrap();

            graph[from as usize * 2].push(to as usize * 2 + 1);
        }

        drop(blue_edges);

        let mut result = vec![-1; n];
        let mut queue = VecDeque::from([0, 1]);
        let mut visited = vec![false; n * 2];
        let mut distance = 1;

        visited[..2].fill(true);
        result[0] = 0;

        loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for &next in &graph[node] {
                    if let visited_value @ false = &mut visited[next] {
                        *visited_value = true;

                        if let distance_value @ -1 = &mut result[next / 2] {
                            *distance_value = distance;
                        }

                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            distance += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::shortest_alternating_paths(n, red_edges, blue_edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
