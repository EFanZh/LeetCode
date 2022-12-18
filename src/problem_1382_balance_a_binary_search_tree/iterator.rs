use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn count_nodes(node: Option<&RefCell<TreeNode>>, result: &mut u32) {
        if let Some(node) = node {
            *result += 1;

            let node = node.borrow();

            Self::count_nodes(node.left.as_deref(), result);
            Self::count_nodes(node.right.as_deref(), result);
        }
    }

    fn iter_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
        let mut state = root;
        let mut stack = Vec::new();

        iter::from_fn(move || {
            while let Some(node) = state.take() {
                state = RefCell::borrow(&node).left.clone();
                stack.push(node);
            }

            stack.pop().map(|node| {
                state = RefCell::borrow(&node).right.clone();
                node
            })
        })
    }

    fn balance(iter: &mut impl Iterator<Item = Rc<RefCell<TreeNode>>>, count: u32) -> Option<Rc<RefCell<TreeNode>>> {
        (count != 0).then(|| {
            let child_nodes = count - 1;
            let left_child_nodes = child_nodes / 2;
            let right_child_nodes = child_nodes - left_child_nodes;
            let left = Self::balance(iter, left_child_nodes);
            let parent = iter.next().unwrap();
            let right = Self::balance(iter, right_child_nodes);

            let mut parent_ref = parent.borrow_mut();

            parent_ref.left = left;
            parent_ref.right = right;

            drop(parent_ref);

            parent
        })
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut total_nodes = 0;

        Self::count_nodes(root.as_deref(), &mut total_nodes);

        Self::balance(&mut Self::iter_nodes(root), total_nodes)
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
