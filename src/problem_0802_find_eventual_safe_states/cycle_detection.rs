pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[Vec<i32>], node: usize, states: &mut [u8]) -> bool {
        let state = &mut states[node];

        if *state == 0 {
            *state = 1;

            for &next in &graph[node] {
                if Self::dfs(graph, next as _, states) {
                    return true;
                }
            }

            states[node] = 2;

            false
        } else {
            *state == 1
        }
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut result = Vec::with_capacity(n);
        let mut states = vec![0; n];

        for node in 0..n {
            if !Self::dfs(&graph, node, &mut states) {
                result.push(node as _);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        Self::eventual_safe_nodes(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
