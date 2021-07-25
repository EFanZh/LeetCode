pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

impl Solution {
    fn get_root(nodes: &mut [(i32, u32)], node: i32) -> (i32, u32) {
        let index = (node - 1) as usize;
        let (parent, rank) = nodes[index];

        if parent == 0 {
            (node, rank)
        } else {
            let (root, root_rank) = Self::get_root(nodes, parent);

            nodes[index].0 = root;

            (root, root_rank)
        }
    }

    fn union(nodes: &mut [(i32, u32)], left_root: i32, left_rank: u32, right_root: i32, right_rank: u32) {
        match left_rank.cmp(&right_rank) {
            Ordering::Less => nodes[(left_root - 1) as usize].0 = right_root,
            Ordering::Equal => {
                nodes[(left_root - 1) as usize].0 = right_root;
                nodes[(right_root - 1) as usize].1 += 1;
            }
            Ordering::Greater => nodes[(right_root - 1) as usize].0 = left_root,
        }
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edges = edges.into_iter();
        let mut nodes = vec![(0, 0); edges.len()];

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
