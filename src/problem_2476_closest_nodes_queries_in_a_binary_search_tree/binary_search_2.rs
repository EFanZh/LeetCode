use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(mut node: Rc<RefCell<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        loop {
            let left = node.borrow().left.clone();

            if let Some(left) = left {
                stack.push(Rc::clone(&node));

                node = left;
            } else {
                loop {
                    let node_ref = node.borrow();
                    let val = node_ref.val;
                    let right = node_ref.right.clone();

                    drop(node_ref);
                    drop(node);

                    result.push(val);

                    if let Some(right) = right {
                        node = right;

                        break;
                    }

                    if let Some(top) = stack.pop() {
                        node = top;
                    } else {
                        return result;
                    }
                }
            }
        }
    }

    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let values = Self::dfs(root.unwrap());

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
