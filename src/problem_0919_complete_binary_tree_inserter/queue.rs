use crate::data_structures::TreeNode;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct CBTInserter {
    root: Rc<RefCell<TreeNode>>,
    slots: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let root = root.unwrap();
        let mut slots = VecDeque::from(vec![Rc::clone(&root)]);

        loop {
            let node = slots.front().unwrap();
            let node = node.borrow();
            let left = node.left.clone();
            let right = node.right.clone();

            drop(node);

            if let Some(left) = left {
                slots.push_back(left);
            } else {
                break;
            }

            if let Some(right) = right {
                slots.push_back(right);
            } else {
                break;
            }

            slots.pop_front();
        }

        Self { root, slots }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let node = Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }));

        let parent = self.slots.front().unwrap();
        let mut parent_ref = parent.borrow_mut();
        let result = parent_ref.val;

        if parent_ref.left.is_none() {
            parent_ref.left = Some(Rc::clone(&node));

            drop(parent_ref);
        } else {
            parent_ref.right = Some(Rc::clone(&node));

            drop(parent_ref);

            self.slots.pop_front();
        }

        self.slots.push_back(node);

        result
    }

    #[allow(clippy::unnecessary_wraps)]
    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::clone(&self.root))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::CBTInserter for CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn insert(&mut self, val: i32) -> i32 {
        self.insert(val)
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.get_root()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CBTInserter>();
    }
}
