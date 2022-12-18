use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn collect_nodes(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let node_ref = node.borrow();
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();

            drop(node_ref);

            Self::collect_nodes(right, result);

            result.push(node);

            Self::collect_nodes(left, result);
        }
    }

    fn build(nodes: &mut Vec<Rc<RefCell<TreeNode>>>, count: usize) -> Option<Rc<RefCell<TreeNode>>> {
        (count != 0).then(|| {
            let child_nodes = count - 1;
            let left_child_nodes = child_nodes / 2;
            let right_child_nodes = child_nodes - left_child_nodes;
            let left = Self::build(nodes, left_child_nodes);
            let parent = nodes.pop().unwrap();
            let right = Self::build(nodes, right_child_nodes);

            let mut parent_ref = parent.borrow_mut();

            parent_ref.left = left;
            parent_ref.right = right;

            drop(parent_ref);

            parent
        })
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut all_nodes = Vec::new();

        Self::collect_nodes(root, &mut all_nodes);

        let count = all_nodes.len();

        Self::build(&mut all_nodes, count)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::balance_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
