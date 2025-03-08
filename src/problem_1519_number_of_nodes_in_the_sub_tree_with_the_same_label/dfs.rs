pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[(u8, Vec<usize>)], counts: &mut [i32; 26], result: &mut [i32], node: usize, parent: usize) {
        let (label, ref children) = graph[node];
        let index = usize::from(label) - usize::from(b'a');
        let old_count = counts[index];

        for &child in children {
            if child != parent {
                Self::dfs(graph, counts, result, child, node);
            }
        }

        let new_count = &mut counts[index];

        *new_count += 1;

        result[node] = *new_count - old_count;
    }

    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as u32 as _;

        let mut graph = labels
            .into_bytes()
            .into_iter()
            .map(|label| (label, Vec::new()))
            .collect::<Box<_>>();

        for edge in edges {
            let [from, to] = edge.try_into().ok().unwrap();
            let from = from as u32 as usize;
            let to = to as u32 as usize;

            graph[from].1.push(to);
            graph[to].1.push(from);
        }

        let mut result = vec![0; n];

        Self::dfs(&graph, &mut [0; 26], &mut result, 0, usize::MAX);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        Self::count_sub_trees(n, edges, labels)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
