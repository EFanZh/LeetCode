pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

struct Node {
    length: usize,
    value: Vec<i32>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Solution {
    fn insert_node(node: &mut Node, index: usize, value: Vec<i32>) {
        node.length += 1;

        if let Some(left) = node.left.as_deref_mut() {
            if index < left.length + 1 {
                Self::insert_node(left, index, value);
            } else {
                Self::insert_tree(&mut node.right, index - (left.length + 1), value);
            }
        } else if index == 0 {
            node.left = Some(Box::new(Node {
                length: 1,
                value,
                left: None,
                right: None,
            }));
        } else {
            Self::insert_tree(&mut node.right, index - 1, value);
        }
    }

    fn insert_tree(tree: &mut Option<Box<Node>>, index: usize, value: Vec<i32>) {
        if let Some(node) = tree.as_deref_mut() {
            Self::insert_node(node, index, value);
        } else {
            *tree = Some(Box::new(Node {
                length: 1,
                value,
                left: None,
                right: None,
            }));
        }
    }

    fn dfs(tree: Option<Box<Node>>, result: &mut Vec<Vec<i32>>) {
        if let Some(node) = tree {
            Self::dfs(node.left, result);
            result.push(node.value);
            Self::dfs(node.right, result);
        }
    }

    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;

        people.sort_unstable_by_key(|p| {
            let [h, k]: [_; 2] = p.as_slice().try_into().unwrap();

            (Reverse(h), k)
        });

        let mut tree = None;

        for p in people.drain(..) {
            let index = p[1] as _;

            Self::insert_tree(&mut tree, index, p);
        }

        Self::dfs(tree, &mut people);

        people
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::reconstruct_queue(people)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
