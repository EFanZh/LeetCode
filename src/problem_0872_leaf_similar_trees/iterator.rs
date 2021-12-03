use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn iter(node: Rc<RefCell<TreeNode>>) -> impl Iterator<Item = i32> {
        let mut stack = vec![node];

        iter::from_fn(move || {
            if let Some(mut node) = stack.pop() {
                loop {
                    let node_ref = node.borrow();

                    match (&node_ref.left, &node_ref.right) {
                        (None, None) => return Some(node_ref.val),
                        (None, Some(child)) | (Some(child), None) => {
                            let new_node = Rc::clone(child);

                            drop(node_ref);

                            node = new_node;
                        }
                        (Some(left), Some(right)) => {
                            let left = Rc::clone(left);
                            let right = Rc::clone(right);

                            drop(node_ref);

                            stack.push(right);
                            node = left;
                        }
                    }
                }
            } else {
                None
            }
        })
    }

    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::iter(root1.unwrap()).eq(Self::iter(root2.unwrap()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::leaf_similar(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
