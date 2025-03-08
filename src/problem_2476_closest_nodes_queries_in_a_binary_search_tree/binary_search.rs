use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(mut node: Rc<RefCell<TreeNode>>, result: &mut Vec<i32>) {
        loop {
            let node_ref = node.borrow();
            let left = node_ref.left.clone();
            let val = node_ref.val;
            let right = node_ref.right.clone();

            drop(node_ref);
            drop(node);

            if let Some(left) = left {
                Self::dfs(left, result);
            }

            result.push(val);

            if let Some(right) = right {
                node = right;
            } else {
                break;
            }
        }
    }

    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut values = Vec::new();

        Self::dfs(root.unwrap(), &mut values);

        queries
            .into_iter()
            .map(|query| {
                let (lower_bound, upper_bound) = match values.binary_search(&query) {
                    Ok(i) => {
                        let value = values[i];

                        (value, value)
                    }
                    Err(i) => (
                        values.get(i.wrapping_sub(1)).copied().unwrap_or(-1),
                        values.get(i).copied().unwrap_or(-1),
                    ),
                };

                vec![lower_bound, upper_bound]
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        Self::closest_nodes(root, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
