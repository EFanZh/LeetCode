use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod level_order_traversal;
pub mod preorder_traversal;

pub trait Codec {
    fn new() -> Self;
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String;
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Codec;

    pub fn run<C: Codec>() {
        let test_cases = [
            &[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)] as &[_],
            &[],
            &[Some(1)],
            &[Some(1), Some(2)],
        ];

        for root in test_cases {
            let codec = C::new();
            let root = test_utilities::make_tree(root.iter().copied());
            let data = codec.serialize(root.clone());
            let recovered_root = codec.deserialize(data);

            assert_eq!(recovered_root, root);
        }
    }
}
