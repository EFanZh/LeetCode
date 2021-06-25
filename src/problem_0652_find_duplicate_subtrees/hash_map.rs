use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        cache: &mut HashMap<(i32, usize, usize), (usize, bool)>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> usize {
        node.map_or(0, |node| {
            let node_ref = node.borrow();
            let left_id = Self::dfs(node_ref.left.as_ref(), cache, result);
            let right_id = Self::dfs(node_ref.right.as_ref(), cache, result);
            let key = (node_ref.val, left_id, right_id);

            drop(node_ref);

            let candidate_id = cache.len() + 1;

            cache
                .entry(key)
                .and_modify(|(_, seen_twice)| {
                    if !*seen_twice {
                        *seen_twice = true;
                        result.push(Some(Rc::clone(node)));
                    }
                })
                .or_insert((candidate_id, false))
                .0
        })
    }

    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();

        Self::dfs(root.as_ref(), &mut HashMap::new(), &mut result);

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
