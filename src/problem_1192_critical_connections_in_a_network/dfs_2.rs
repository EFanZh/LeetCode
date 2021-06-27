pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    fn dfs(
        graph: &[Vec<i32>],
        vertex: i32,
        parent: i32,
        time: u32,
        states: &mut [u32],
        result: &mut Vec<Vec<i32>>,
    ) -> u32 {
        let mut low = time;

        states[vertex as usize] = time;

        for &next in &graph[vertex as usize] {
            if next != parent {
                let state = &mut states[next as usize];

                if *state == u32::MAX {
                    let next_low = Self::dfs(graph, next, vertex, time + 1, states, result);

                    if next_low > time {
                        result.push(vec![vertex, next]);
                    }

                    low = low.min(next_low);
                } else {
                    low = low.min(*state);
                }
            }
        }

        states[vertex as usize] = low;

        low
    }

    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); n as _];

        for [from, to] in connections.iter().map(Vec::as_slice).flat_map(<[i32; 2]>::try_from) {
            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut result = Vec::new();

        Self::dfs(&graph, 0, -1, 0, &mut vec![u32::MAX; graph.len()], &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::critical_connections(n, connections)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
