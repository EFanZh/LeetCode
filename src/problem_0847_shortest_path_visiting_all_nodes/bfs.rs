pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len() as u16;

        if n < 4 {
            (n - 1).into()
        } else {
            let unvisited_mask = (1 << n) - 1;

            let mut queue = (0..n)
                .map(|node| (node << n) | (!(1 << node)) & unvisited_mask)
                .collect::<VecDeque<_>>();

            let mut visited = vec![false; usize::from(*queue.back().unwrap()) - 1];

            for &state in &queue {
                visited[usize::from(state) - 2] = true;
            }

            let mut result = 1;

            loop {
                for _ in 0..queue.len() {
                    let state = queue.pop_front().unwrap();
                    let node = state >> n;
                    let unvisited = state & unvisited_mask;

                    for &next_node in &graph[usize::from(node)] {
                        let next_node = next_node as u16;
                        let next_unvisited = unvisited & !(1 << next_node);

                        if next_unvisited == 0 {
                            return result;
                        }

                        let next_state = (next_node << n) | next_unvisited;

                        if let visited_value @ false = &mut visited[usize::from(next_state) - 2] {
                            *visited_value = true;

                            queue.push_back(next_state);
                        }
                    }
                }

                result += 1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        Self::shortest_path_length(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
