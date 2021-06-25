use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

struct Node {
    inner: Rc<RefCell<TreeNode>>,
    hash_value: u64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.inner.as_ref(), other.inner.as_ref())
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_value);
    }
}

impl Solution {
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        cache: &mut HashMap<Node, bool>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> u64 {
        node.map_or(0, |node| {
            let node_ref = node.borrow();
            let left_hash = Self::dfs(node_ref.left.clone(), cache, result);
            let right_hash = Self::dfs(node_ref.right.clone(), cache, result);

            let hash_key = (node_ref.val, left_hash, right_hash);

            drop(node_ref);

            let mut hasher = DefaultHasher::default();

            hash_key.hash(&mut hasher);

            let hash_value = hasher.finish();

            cache
                .entry(Node {
                    inner: Rc::clone(&node),
                    hash_value,
                })
                .and_modify(move |seen_twice| {
                    if !*seen_twice {
                        *seen_twice = true;
                        result.push(Some(node));
                    }
                })
                .or_insert(false);

            hash_value
        })
    }

    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();

        Self::dfs(root, &mut HashMap::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::find_duplicate_subtrees(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
