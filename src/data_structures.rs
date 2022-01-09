use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

#[allow(variant_size_differences)]
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NestedInteger, TreeNode};

    #[test]
    fn test_debug_tree_node() {
        assert_eq!(
            format!("{:?}", TreeNode::new(2)),
            "TreeNode { val: 2, left: None, right: None }"
        );
    }

    #[test]
    fn test_debug_nested_integer() {
        assert_eq!(format!("{:?}", NestedInteger::Int(2)), "Int(2)");

        assert_eq!(
            format!(
                "{:?}",
                NestedInteger::List(vec![NestedInteger::Int(3), NestedInteger::Int(5)])
            ),
            "List([Int(3), Int(5)])"
        );
    }
}
