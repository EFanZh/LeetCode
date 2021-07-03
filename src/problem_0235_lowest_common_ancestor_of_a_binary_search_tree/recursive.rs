use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(root: &Rc<RefCell<TreeNode>>, p: &TreeNode, q: &TreeNode) -> Rc<RefCell<TreeNode>> {
        let root_ref = root.borrow();

        if q.val < root_ref.val {
            Self::helper(root_ref.left.as_ref().unwrap(), p, q)
        } else if p.val > root_ref.val {
            Self::helper(root_ref.right.as_ref().unwrap(), p, q)
        } else {
            Rc::clone(root)
        }
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();
        let p = p.unwrap();
        let q = q.unwrap();
        let p_ref = p.borrow();
        let q_ref = q.borrow();

        Some(if p_ref.val < q_ref.val {
            Self::helper(&root, &p_ref, &q_ref)
        } else {
            Self::helper(&root, &q_ref, &p_ref)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor(root, p, q)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
