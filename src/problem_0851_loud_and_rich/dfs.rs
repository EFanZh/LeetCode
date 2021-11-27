pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn dfs(graph: &[Vec<usize>], quiet: &[i32], node: usize, loudest: &mut [i32]) -> usize {
        let mut result = loudest[node] as _;

        if result == usize::MAX {
            result = node;

            for &next in &graph[node] {
                let next_result = Self::dfs(graph, quiet, next, loudest);

                if quiet[next_result] < quiet[result] {
                    result = next_result;
                }
            }

            loudest[node] = result as _;
        }

        result
    }

    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph = vec![Vec::new(); n];

        for edge in &richer {
            let [rich, poor]: [i32; 2] = edge.as_slice().try_into().unwrap();

            graph[poor as usize].push(rich as usize);
        }

        let mut result = vec![-1; n];

        for node in 0..n {
            Self::dfs(&graph, &quiet, node, &mut result);
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
