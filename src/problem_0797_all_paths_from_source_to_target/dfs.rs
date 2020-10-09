pub struct Solution;

impl Solution {
    fn dfs<'a>(graph: &[Vec<i32>], node: i32, cache: &'a mut Vec<Option<Vec<Vec<i32>>>>) -> &'a [Vec<i32>] {
        if cache[node as usize].is_none() {
            let mut node_paths = Vec::new();

            if node == graph.len() as i32 - 1 {
                node_paths.push(vec![node]);
            } else {
                for &next in &graph[node as usize] {
                    for next_path in Self::dfs(graph, next, cache) {
                        let mut path = Vec::with_capacity(next_path.len() + 1);

                        path.push(node);
                        path.extend_from_slice(next_path);

                        node_paths.push(path);
                    }
                }
            }

            cache[node as usize] = Some(node_paths);
        }

        cache[node as usize].as_deref().unwrap()
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cache = vec![None; graph.len()];

        Self::dfs(&graph, 0, &mut cache);

        cache[0].take().unwrap()
    }
}

impl super::Solution for Solution {
    fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::all_paths_source_target(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
