use crate::data_structures::TreeNode;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

pub struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize_helper(root: Option<&RefCell<TreeNode>>, result: &mut String) {
        use std::fmt::Write;

        if let Some(node) = root {
            let node_ref = node.borrow();

            write!(result, "{} ", node_ref.val).unwrap();

            Self::serialize_helper(node_ref.left.as_deref(), result);

            result.push(' ');

            Self::serialize_helper(node_ref.right.as_deref(), result);
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let _ = self;
        let mut result = String::new();

        Self::serialize_helper(root.as_deref(), &mut result);

        result
    }

    fn deserialize_helper(data: &mut impl Iterator<Item = impl AsRef<str>>) -> Option<Rc<RefCell<TreeNode>>> {
        data.next().unwrap().as_ref().parse().ok().map(|val| {
            Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::deserialize_helper(data),
                right: Self::deserialize_helper(data),
            }))
        })
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let _ = self;

        Self::deserialize_helper(&mut data.split(' '))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Codec for Codec {
    fn new() -> Self {
        Self::new()
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.serialize(root)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.deserialize(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Codec>();
    }
}
