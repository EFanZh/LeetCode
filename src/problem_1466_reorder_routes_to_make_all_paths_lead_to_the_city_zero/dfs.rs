pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[Vec<usize>], node: usize, parent: usize, count: &mut i32) {
        for &neighbor in &graph[node] {
            let real_neighbor = neighbor & (usize::MAX >> 1);

            if real_neighbor != parent {
                *count += i32::from(neighbor & !(usize::MAX >> 1) == 0);

                Self::dfs(graph, real_neighbor, node, count);
            }
        }
    }

    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![Vec::new(); n as _];

        for connection in connections {
            let [from, to]: [_; 2] = connection.try_into().ok().unwrap();
            let from = from as usize;
            let to = to as usize;

            graph[from].push(to);
            graph[to].push(from | !(usize::MAX >> 1));
        }

        let mut result = 0;

        Self::dfs(&graph, 0, usize::MAX >> 1, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        Self::min_reorder(n, connections)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
