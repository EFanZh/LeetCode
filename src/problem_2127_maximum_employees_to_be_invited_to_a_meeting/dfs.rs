pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn longest_cycle(favorite: &[u32]) -> u32 {
        let n = favorite.len();
        let mut result = 0;
        let mut states = vec![(0_u32, 0_u32); n];

        for i in 0..n {
            let state = &mut states[i];

            if state.0 == 0 {
                let group = i as u32 + 1;
                let mut order = 0;
                let mut node = favorite[i];

                state.0 = group;

                loop {
                    let node_usize = node as usize;
                    let state = &mut states[node_usize];

                    if state.0 == 0 {
                        order += 1;
                        *state = (group, order);
                        node = favorite[node_usize];
                    } else {
                        if state.0 == group {
                            result = result.max(order - state.1);
                        }

                        break;
                    }
                }
            }
        }

        result + 1
    }

    fn tree_depth(tree: &[Vec<u32>], node: u32) -> u32 {
        let mut max_depth = 0;

        for &child in &tree[node as usize] {
            max_depth = max_depth.max(Self::tree_depth(tree, child));
        }

        max_depth + 1
    }

    fn case_2(favorite: &[u32]) -> u32 {
        let n = favorite.len();
        let mut graph = vec![Vec::new(); n];

        for (child, &parent) in (0..).zip(favorite) {
            graph[parent as usize].push(child);
        }

        let mut result = 0;

        for ((node, &other), children) in (0..).zip(favorite).zip(&graph) {
            if node < other && favorite[other as usize] == node {
                let mut max_depth_1 = 0;

                for &child in children {
                    if child != other {
                        max_depth_1 = max_depth_1.max(Self::tree_depth(&graph, child));
                    }
                }

                let mut max_depth_2 = 0;

                for &child in &graph[other as usize] {
                    if child != node {
                        max_depth_2 = max_depth_2.max(Self::tree_depth(&graph, child));
                    }
                }

                result += max_depth_1 + max_depth_2 + 2;
            }
        }

        result
    }

    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let favorite = favorite.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let size_1 = Self::longest_cycle(&favorite);
        let size_2 = Self::case_2(&favorite);

        drop(favorite);

        size_1.max(size_2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        Self::maximum_invitations(favorite)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
