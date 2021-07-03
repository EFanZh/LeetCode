use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn merge_to(target: &mut Option<Rc<RefCell<TreeNode>>>, tree: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(tree) = tree {
            if let Some(target) = target {
                let mut target = target.borrow_mut();
                let mut tree = tree.borrow_mut();

                target.val += tree.val;

                Self::merge_to(&mut target.left, tree.left.take());
                Self::merge_to(&mut target.right, tree.right.take());
            } else {
                *target = Some(tree);
            }
        }
    }

    pub fn merge_trees(
        mut root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::merge_to(&mut root1, root2);

        root1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::merge_trees(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
