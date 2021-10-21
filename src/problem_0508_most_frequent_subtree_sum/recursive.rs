use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn helper(node: Option<&TreeNode>, count: &mut HashMap<i32, u16>) -> i32 {
        node.map_or(0, |node| {
            let left_sum = Self::helper(node.left.as_deref().map(RefCell::borrow).as_deref(), count);
            let right_sum = Self::helper(node.right.as_deref().map(RefCell::borrow).as_deref(), count);
            let sum = left_sum + right_sum + node.val;

            count.entry(sum).and_modify(|c| *c += 1).or_insert(1);

            sum
        })
    }

    #[allow(clippy::if_then_some_else_none)]
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut count = HashMap::new();

        Self::helper(root.as_deref().map(RefCell::borrow).as_deref(), &mut count);

        let max_count = count.values().copied().max().unwrap();

        count
            .into_iter()
            .filter_map(|(k, v)| if v == max_count { Some(k) } else { None })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::find_frequent_tree_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
