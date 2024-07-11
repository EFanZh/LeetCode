pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[(Vec<usize>, bool)], parent: usize, node: usize) -> i32 {
        let (neighbors, has_apple) = &graph[node];
        let has_apple = *has_apple;
        let mut total_cost = 0;

        for &neighbor in neighbors {
            if neighbor != parent {
                total_cost += Self::dfs(graph, node, neighbor);
            }
        }

        if !has_apple && total_cost == 0 {
            0
        } else {
            total_cost + 2
        }
    }

    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let _ = n;

        let mut graph = has_apple
            .into_iter()
            .map(|has_apple| (Vec::new(), has_apple))
            .collect::<Vec<_>>();

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from as usize;
            let to = to as usize;

            graph[from].0.push(to);
            graph[to].0.push(from);
        }

        let graph = graph.as_slice();
        let mut result = 0;

        for &neighbor in &graph[0].0 {
            result += Self::dfs(graph, 0, neighbor);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        Self::min_time(n, edges, has_apple)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
