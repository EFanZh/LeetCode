pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

impl Solution {
    fn find_root(union_find: &mut [(u32, u32)], node: u32) -> u32 {
        let parent = union_find[node as usize].0;

        if parent == u32::MAX {
            node
        } else {
            let root = Self::find_root(union_find, parent);

            union_find[node as usize].0 = root;

            root
        }
    }

    fn union(union_find: &mut [(u32, u32)], left: u32, right: u32) -> bool {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if left_root == right_root {
            false
        } else {
            match union_find[left_root as usize].1.cmp(&union_find[right_root as usize].1) {
                Ordering::Less => union_find[left_root as usize].0 = right_root,
                Ordering::Equal => {
                    union_find[left_root as usize].1 += 1;
                    union_find[right_root as usize].0 = left_root;
                }
                Ordering::Greater => union_find[right_root as usize].0 = left_root,
            }

            true
        }
    }

    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;
        let total_edges = edges.len();
        let mut alice_edges = Vec::new();
        let mut bob_edges = Vec::new();
        let mut shared_edges = Vec::new();

        for edge in edges {
            let [edge_type, from, to]: [_; 3] = edge.as_slice().try_into().ok().unwrap();

            let container = match edge_type {
                1 => &mut alice_edges,
                2 => &mut bob_edges,
                _ => &mut shared_edges,
            };

            container.push((from as u32 - 1, to as u32 - 1));
        }

        // Shared edges.

        let mut shared_edge_count = 0;

        let mut alice_union_find = vec![(u32::MAX, 0); n].into_boxed_slice();

        for edge in shared_edges {
            shared_edge_count += usize::from(Self::union(&mut alice_union_find, edge.0, edge.1));
        }

        let bob_union_find = alice_union_find.clone();

        // Individual edges.

        let mut individual_edge_count = 0;

        for (edges, mut union_find) in [(alice_edges, alice_union_find), (bob_edges, bob_union_find)] {
            let mut edge_count = 0;

            for edge in edges {
                edge_count += usize::from(Self::union(&mut union_find, edge.0, edge.1));
            }

            if shared_edge_count + edge_count < n - 1 {
                return -1;
            }

            individual_edge_count += edge_count;
        }

        (total_edges - (shared_edge_count + individual_edge_count)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::max_num_edges_to_remove(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
