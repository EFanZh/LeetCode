pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum ChildrenDepths {
    None,
    Single(u8),
    Multiple(u8, u8),
}

impl Solution {
    fn dfs(graph: &[u16; 16], visited: &mut u16, node: u8) -> (u8, u8) {
        let mut children = graph[usize::from(node)];
        let mut max_distance = 0;
        let mut max_depth = 0;
        let mut children_depths = ChildrenDepths::None;

        while children != 0 {
            let child = children.trailing_zeros() as _;

            children &= children - 1;

            let probe = 1_u16 << child;

            if *visited & probe == 0 {
                *visited |= probe;

                let (child_max_distance, child_depth) = Self::dfs(graph, visited, child);

                max_distance = max_distance.max(child_max_distance);
                max_depth = max_depth.max(child_depth + 1);

                children_depths = match children_depths {
                    ChildrenDepths::None => ChildrenDepths::Single(child_depth),
                    ChildrenDepths::Single(depth) => {
                        if child_depth > depth {
                            ChildrenDepths::Multiple(child_depth, depth)
                        } else {
                            ChildrenDepths::Multiple(depth, child_depth)
                        }
                    }
                    ChildrenDepths::Multiple(existing_depth_1, existing_depth_2) => {
                        if child_depth > existing_depth_2 {
                            if child_depth > existing_depth_1 {
                                ChildrenDepths::Multiple(child_depth, existing_depth_1)
                            } else {
                                ChildrenDepths::Multiple(existing_depth_1, child_depth)
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }

        (
            max_distance.max(match children_depths {
                ChildrenDepths::None => 0,
                ChildrenDepths::Single(depth) => depth + 1,
                ChildrenDepths::Multiple(depth_1, depth_2) => depth_1 + depth_2 + 2,
            }),
            max_depth,
        )
    }

    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as u32 as usize;
        let mut graph = [0_u16; 16];

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from - 1;
            let to = to - 1;

            graph[from as u32 as usize] |= 1 << to;
            graph[to as u32 as usize] |= 1 << from;
        }

        let mut result = vec![0; n - 1];

        for subset in 3_u16..(1 << n) {
            if !subset.is_power_of_two() {
                let mut subgraph = graph;

                for neighbor in &mut subgraph[..n] {
                    *neighbor &= subset;
                }

                let node = subset.trailing_zeros() as _;
                let mut visited = 1_u16 << node;
                let candidate_max_distance = Self::dfs(&subgraph, &mut visited, node).0;

                if visited == subset {
                    result[usize::from(candidate_max_distance) - 1] += 1;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::count_subgraphs_for_each_diameter(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
