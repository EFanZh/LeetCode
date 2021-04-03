use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &TreeNode, result: &mut String) {
        use std::fmt::Write;

        write!(result, "{}", node.val).unwrap();

        match (
            node.left.as_deref().map(RefCell::borrow),
            node.right.as_deref().map(RefCell::borrow),
        ) {
            (None, None) => {}
            (None, Some(right)) => {
                result.push_str("()(");
                Self::helper(&right, result);
                result.push(')');
            }
            (Some(left), None) => {
                result.push('(');
                Self::helper(&left, result);
                result.push(')');
            }
            (Some(left), Some(right)) => {
                result.push('(');
                Self::helper(&left, result);
                result.push_str(")(");
                Self::helper(&right, result);
                result.push(')');
            }
        }
    }

    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();

        if let Some(node) = t {
            Self::helper(&node.borrow(), &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::tree2str(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
