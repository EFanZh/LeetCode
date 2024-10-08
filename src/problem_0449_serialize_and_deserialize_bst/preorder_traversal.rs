use crate::data_structures::TreeNode;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

pub struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize_helper(root: Option<&RefCell<TreeNode>>, result: &mut Vec<u8>) {
        if let Some(node) = root {
            let node_ref = node.borrow();

            result.push((node_ref.val / 128) as _);
            result.push((node_ref.val % 128) as _);

            Self::serialize_helper(node_ref.left.as_deref(), result);
            Self::serialize_helper(node_ref.right.as_deref(), result);
        }
    }

    #[expect(clippy::unused_self, reason = "required")]
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();

        Self::serialize_helper(root.as_deref(), &mut result);

        String::from_utf8(result).unwrap()
    }

    fn deserialize_helper(
        data: &mut Peekable<impl Iterator<Item = i32>>,
        lower: i32,
        upper: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        data.peek()
            .filter(|&&val| val >= lower && val <= upper)
            .copied()
            .map(|val| {
                data.next();

                Rc::new(RefCell::new(TreeNode {
                    val,
                    left: Self::deserialize_helper(data, lower, val),
                    right: Self::deserialize_helper(data, val, upper),
                }))
            })
    }

    #[expect(clippy::unused_self, reason = "required")]
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data
            .bytes()
            .zip(data.bytes().skip(1))
            .step_by(2)
            .map(|(high, low)| 128 * i32::from(high) + i32::from(low))
            .peekable();

        Self::deserialize_helper(&mut iter, i32::MIN, i32::MAX)
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
