pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn get_root(roots: &mut [i32], node: i32) -> i32 {
        let index = (node - 1) as usize;
        let parent = roots[index];

        if parent == 0 {
            node
        } else {
            let root = Self::get_root(roots, parent);

            roots[index] = root;

            root
        }
    }

    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut roots = vec![0; edges.len()];
        let mut parents = vec![0; edges.len()];
        let mut first = [0, 0];
        let mut second = [0, 0];
        let mut last = [0, 0];

        for edge in edges {
            let [from, to]: [i32; 2] = edge.as_slice().try_into().unwrap();
            let parent = &mut parents[(to - 1) as usize];

            if *parent == 0 {
                *parent = from;

                let root = Self::get_root(&mut roots, from);

                if root == to {
                    last = [from, to];
                } else {
                    roots[(to - 1) as usize] = from;
                }
            } else {
                first = [*parent, to];
                second = [from, to];
            }
        }

        if first == [0, 0] {
            last.to_vec()
        } else if last == [0, 0] {
            second.to_vec()
        } else {
            first.to_vec()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_redundant_directed_connection(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
