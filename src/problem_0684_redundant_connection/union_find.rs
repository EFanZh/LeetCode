pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    fn get_root(nodes: &mut HashMap<i32, (i32, u32)>, node: i32) -> (i32, u32) {
        match nodes.entry(node) {
            Entry::Occupied(entry) => {
                let (parent, rank) = *entry.get();

                if parent == 0 {
                    (node, rank)
                } else {
                    let (root, root_rank) = Self::get_root(nodes, parent);

                    nodes.get_mut(&node).unwrap().0 = root;

                    (root, root_rank)
                }
            }
            Entry::Vacant(entry) => {
                entry.insert((0, 0));

                (node, 0)
            }
        }
    }

    fn union(nodes: &mut HashMap<i32, (i32, u32)>, left_root: i32, left_rank: u32, right_root: i32, right_rank: u32) {
        match left_rank.cmp(&right_rank) {
            Ordering::Less => nodes.get_mut(&left_root).unwrap().0 = right_root,
            Ordering::Equal => {
                nodes.get_mut(&left_root).unwrap().0 = right_root;
                nodes.get_mut(&right_root).unwrap().1 += 1;
            }
            Ordering::Greater => nodes.get_mut(&right_root).unwrap().0 = left_root,
        }
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edges = edges.into_iter();
        let mut nodes = HashMap::new();

        loop {
            let edge = edges.next().unwrap();
            let [a, b]: [i32; 2] = edge.as_slice().try_into().unwrap();
            let (left_root, left_rank) = Self::get_root(&mut nodes, a);
            let (right_root, right_rank) = Self::get_root(&mut nodes, b);

            if left_root == right_root {
                return edge;
            }

            Self::union(&mut nodes, left_root, left_rank, right_root, right_rank);
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_redundant_connection(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
