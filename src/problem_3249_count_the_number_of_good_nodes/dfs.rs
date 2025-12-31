pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[Vec<u32>], result: &mut u32, node: u32, parent: u32) -> u32 {
        let mut size = 1;

        let mut iter = graph[node as usize]
            .iter()
            .copied()
            .filter(|&child| child != parent)
            .map(|child| Self::dfs(graph, result, child, node))
            .inspect(|&child_size| size += child_size);

        *result += u32::from(iter.next().is_none_or(|first_child_size| {
            let all_same = iter.all(|child_size| child_size == first_child_size);

            iter.for_each(drop);

            all_same
        }));

        size
    }

    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![Vec::new(); edges.len() + 1].into_boxed_slice();

        for edge in edges {
            let [x, y] = <[_; 2]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[x as usize].push(y);
            graph[y as usize].push(x);
        }

        let mut result = 0;

        Self::dfs(&graph, &mut result, 0, u32::MAX);

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        Self::count_good_nodes(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
