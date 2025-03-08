pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        if connections.len() < n - 1 {
            -1
        } else {
            let mut graph = vec![Vec::new(); n];

            for connection in connections {
                let [from, to] = connection.try_into().ok().unwrap();
                let from = from as usize;
                let to = to as usize;

                graph[from].push(to);
                graph[to].push(from);
            }

            let mut queue = VecDeque::new();
            let mut visited = vec![false; n];
            let mut components = 0;

            for node in 0..n {
                if let visited_value @ false = &mut visited[node] {
                    *visited_value = true;

                    components += 1;

                    let mut node = node;

                    loop {
                        for &next in &graph[node] {
                            if let visited_value @ false = &mut visited[next] {
                                *visited_value = true;
                                queue.push_back(next);
                            }
                        }

                        if let Some(next) = queue.pop_front() {
                            node = next;
                        } else {
                            break;
                        }
                    }
                }
            }

            components - 1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        Self::make_connected(n, connections)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
