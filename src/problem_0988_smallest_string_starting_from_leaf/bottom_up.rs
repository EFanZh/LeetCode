use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn dfs(node: &RefCell<TreeNode>, parent: u16, parents: &mut Vec<(u16, u16)>, leaves: &mut Vec<u16>) {
        let node = node.borrow();
        let id = parents.len() as _;

        parents.push((parent, node.val as _));

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => leaves.push(id),
            (None, Some(child)) | (Some(child), None) => Self::dfs(child, id, parents, leaves),
            (Some(left), Some(right)) => {
                Self::dfs(left, id, parents, leaves);
                Self::dfs(right, id, parents, leaves);
            }
        }
    }

    fn make_iter(parents: &[(u16, u16)], mut node: u16) -> impl Iterator<Item = u16> + '_ {
        iter::from_fn(move || {
            parents.get(usize::from(node)).map(|&(parent, value)| {
                node = parent;

                value
            })
        })
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut parents = Vec::new();
        let mut leaves = Vec::new();

        Self::dfs(root.as_deref().unwrap(), u16::MAX, &mut parents, &mut leaves);

        let parents = parents.as_slice();

        leaves
            .iter()
            .map(|&id| move || Self::make_iter(parents, id))
            .min_by(|lhs, rhs| lhs().cmp(rhs()))
            .unwrap()()
        .map(|value| char::from(b'a' + value as u8))
        .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::smallest_from_leaf(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
