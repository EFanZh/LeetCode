use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashMap::new();
        let mut children = HashSet::new();

        let mut get_node = |value| {
            Rc::clone(
                nodes
                    .entry(value)
                    .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(value)))),
            )
        };

        for description in descriptions {
            let [parent, child, is_left] = description.try_into().ok().unwrap();

            {
                let parent_node = get_node(parent);
                let mut parent_node = parent_node.borrow_mut();
                let parent_node = &mut *parent_node;
                let child_node = get_node(child);

                (if is_left == 0 {
                    &mut parent_node.right
                } else {
                    &mut parent_node.left
                })
                .replace(child_node);
            }

            children.insert(child);
        }

        nodes.into_values().find(|node| !children.contains(&node.borrow().val))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::create_binary_tree(descriptions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
