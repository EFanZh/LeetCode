pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// <https://leetcode.com/problems/sum-of-distances-in-tree/solution/>.

use std::convert::TryInto;

impl Solution {
    fn dfs(graph: &[Vec<usize>], node: usize, parent: usize, counts: &mut [i32]) -> (i32, i32) {
        let mut count = 1;
        let mut total_distance = 0;

        for &child in &graph[node] {
            if child != parent {
                let (child_count, child_sum) = Self::dfs(graph, child, node, counts);

                count += child_count;
                total_distance += child_count + child_sum;
            }
        }

        counts[node] = count;

        (count, total_distance)
    }

    fn dfs_2(
        graph: &[Vec<usize>],
        node: usize,
        parent: usize,
        total_distance: i32,
        counts: &[i32],
        result: &mut [i32],
    ) {
        result[node] = total_distance;

        let n = graph.len() as i32;

        for &child in &graph[node] {
            if child != parent {
                let left_count = counts[child];
                let right_count = n - left_count;
                let diff = right_count - left_count;

                Self::dfs_2(graph, child, node, total_distance + diff, counts, result);
            }
        }
    }

    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as _;
        let mut graph = vec![Vec::new(); n];

        for edge in &edges {
            let [from, to]: [_; 2] = edge.as_slice().try_into().unwrap();
            let (from, to) = (from as usize, to as usize);

            graph[from].push(to);
            graph[to].push(from);
        }

        let mut counts = vec![0; n];
        let root_total_distance = Self::dfs(&graph, 0, usize::MAX, &mut counts).1;
        let mut result = vec![0; n];

        Self::dfs_2(&graph, 0, usize::MAX, root_total_distance, &counts, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::sum_of_distances_in_tree(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
