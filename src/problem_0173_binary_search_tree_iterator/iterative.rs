use super::super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();

        while let Some(node) = root {
            root = node.borrow().left.clone();

            stack.push(node);
        }

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let node_ref = node.borrow();
        let mut root = node_ref.right.clone();

        while let Some(node) = root {
            root = node.borrow().left.clone();

            self.stack.push(node);
        }

        node_ref.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

impl super::BSTIterator for BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn next(&mut self) -> i32 {
        self.next()
    }

    fn has_next(&self) -> bool {
        self.has_next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::BSTIterator>();
    }
}
