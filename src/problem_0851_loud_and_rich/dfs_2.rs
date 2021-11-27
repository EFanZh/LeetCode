pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn dfs(graph: &[(i32, Vec<usize>)], node: usize, loudest: &mut [i32]) -> (usize, i32) {
        let result_node = loudest[node] as _;

        if result_node == usize::MAX {
            let (result_quiet, nexts) = &graph[node];
            let mut result = (node, *result_quiet);

            for &next in nexts {
                let next_result = Self::dfs(graph, next, loudest);

                if next_result.1 < result.1 {
                    result = next_result;
                }
            }

            loudest[node] = result.0 as _;

            result
        } else {
            (result_node, graph[result_node].0)
        }
    }

    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph = quiet.into_iter().map(|quiet| (quiet, Vec::new())).collect::<Vec<_>>();

        for edge in &richer {
            let [rich, poor]: [i32; 2] = edge.as_slice().try_into().unwrap();

            graph[poor as usize].1.push(rich as usize);
        }

        let mut result = vec![-1; n];

        for node in 0..n {
            Self::dfs(&graph, node, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        Self::loud_and_rich(richer, quiet)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
