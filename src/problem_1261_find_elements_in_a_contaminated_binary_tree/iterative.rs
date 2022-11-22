use crate::data_structures::TreeNode;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

pub struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn find(&self, target: i32) -> bool {
        let target = target + 1;
        let mut node = self.root.clone();
        let mut probe = 1 << (31 - target.leading_zeros());

        loop {
            probe >>= 1;

            if probe == 0 {
                break;
            }

            if let Some(node_rc) = node {
                let node_ref = node_rc.borrow();

                let child = if target & probe == 0 {
                    &node_ref.left
                } else {
                    &node_ref.right
                };

                node = child.clone();
            } else {
                return false;
            }
        }

        node.is_some()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FindElements for FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn find(&self, target: i32) -> bool {
        self.find(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FindElements>();
    }
}
