use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn helper(
        node: Option<&RefCell<TreeNode>>,
        sum: i32,
        mut prefix_sum: i32,
        prefix_sums: &mut HashMap<i32, i32>,
        result: &mut i32,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            let left = node.left.as_deref();
            let right = node.right.as_deref();

            prefix_sum += node.val;

            if let Some(count) = prefix_sums.get(&(prefix_sum - sum)) {
                *result += count;
            }

            match prefix_sums.entry(prefix_sum) {
                Entry::Occupied(entry) => {
                    *entry.into_mut() += 1;

                    Self::helper(left, sum, prefix_sum, prefix_sums, result);
                    Self::helper(right, sum, prefix_sum, prefix_sums, result);

                    *prefix_sums.get_mut(&prefix_sum).unwrap() -= 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);

                    Self::helper(left, sum, prefix_sum, prefix_sums, result);
                    Self::helper(right, sum, prefix_sum, prefix_sums, result);

                    prefix_sums.remove(&prefix_sum);
                }
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), sum, 0, &mut iter::once((0, 1)).collect(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        Self::path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
