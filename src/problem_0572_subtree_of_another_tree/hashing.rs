use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

impl Solution {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::default();

        value.hash(&mut hasher);

        hasher.finish()
    }

    fn hash_tree(tree: Option<&RefCell<TreeNode>>) -> u64 {
        Self::hash(&tree.map(|node| {
            let node = node.borrow();

            (
                node.val,
                Self::hash_tree(node.left.as_deref()),
                Self::hash_tree(node.right.as_deref()),
            )
        }))
    }

    fn helper(s: Option<&RefCell<TreeNode>>, t: Option<&RefCell<TreeNode>>, target_hash: u64) -> Option<u64> {
        let hash = Self::hash(&if let Some(node) = s {
            let node = node.borrow();

            Some((
                node.val,
                Self::helper(node.left.as_deref(), t, target_hash)?,
                Self::helper(node.right.as_deref(), t, target_hash)?,
            ))
        } else {
            None
        });

        if hash == target_hash && s == t {
            None
        } else {
            Some(hash)
        }
    }

    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let target_hash = Self::hash_tree(t.as_deref());

        Self::helper(s.as_deref(), t.as_deref(), target_hash).is_none()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_subtree(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
