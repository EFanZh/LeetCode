use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root.unwrap();
        let mut p = p.unwrap().borrow().val;
        let mut q = q.unwrap().borrow().val;

        if p > q {
            mem::swap(&mut p, &mut q);
        }

        loop {
            root = {
                let root_ref = root.borrow();

                if q < root_ref.val {
                    root_ref.left.clone().unwrap()
                } else if p > root_ref.val {
                    root_ref.right.clone().unwrap()
                } else {
                    break;
                }
            }
        }

        Some(root)
    }
}

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
