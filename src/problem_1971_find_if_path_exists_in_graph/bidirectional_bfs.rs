pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

enum State {
    NotVisited(Vec<u32>),
    Left,
    Right,
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            true
        } else {
            let source = source as u32;
            let destination = destination as u32;
            let mut graph = vec![Vec::new(); n as u32 as usize];

            for edge in edges {
                let [from, to] = edge.try_into().ok().unwrap();
                let from = from as u32;
                let to = to as u32;

                graph[from as usize].push(to);
                graph[to as usize].push(from);
            }

            let mut left_neighbors = mem::take(&mut graph[source as usize]);
            let mut right_neighbors = mem::take(&mut graph[destination as usize]);
            let mut states = graph.into_iter().map(State::NotVisited).collect::<Box<_>>();

            states[source as usize] = State::Left;
            states[destination as usize] = State::Right;

            let mut left_queue = VecDeque::new();
            let mut right_queue = VecDeque::new();

            loop {
                if right_queue.len() < left_queue.len() {
                    for child in right_neighbors {
                        match mem::replace(&mut states[child as usize], State::Right) {
                            State::NotVisited(next_neighbors) => right_queue.push_back(next_neighbors),
                            State::Left => return true,
                            State::Right => {}
                        }
                    }

                    if let Some(next_right_neighbors) = right_queue.pop_front() {
                        right_neighbors = next_right_neighbors;
                    } else {
                        break;
                    }
                } else {
                    for child in left_neighbors {
                        match mem::replace(&mut states[child as usize], State::Left) {
                            State::NotVisited(next_neighbors) => left_queue.push_back(next_neighbors),
                            State::Left => {}
                            State::Right => return true,
                        }
                    }

                    if let Some(next_left_neighbors) = left_queue.pop_front() {
                        left_neighbors = next_left_neighbors;
                    } else {
                        break;
                    }
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
