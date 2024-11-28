use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;
use std::rc::Rc;

impl Solution {
    fn validate_binary_tree_option(prev: i32, node: Option<&RefCell<TreeNode>>) -> Option<i32> {
        node.map_or(Some(prev), |node| Self::validate_binary_tree(prev, node))
    }

    fn validate_binary_tree(mut prev: i32, node: &RefCell<TreeNode>) -> Option<i32> {
        let node = node.borrow();
        let node = &*node;

        prev = Self::validate_binary_tree_option(prev, node.left.as_deref())?;

        if prev < node.val {
            Self::validate_binary_tree_option(node.val, node.right.as_deref())
        } else {
            None
        }
    }

    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let trees = trees.into_iter().flatten().collect::<Vec<_>>();
        let mut nodes = HashMap::new();
        let mut non_roots = HashSet::new();

        for tree in &trees {
            let tree_holder = tree.borrow();
            let tree_ref = &*tree_holder;

            for child in [&tree_ref.left, &tree_ref.right].into_iter().flatten() {
                if !non_roots.insert(child.borrow().val) {
                    return None;
                }
            }

            let tree_val = tree_ref.val;

            drop(tree_holder);

            nodes.insert(tree_val, Rc::clone(tree));
        }

        nodes
            .keys()
            .copied()
            .find(|key| !non_roots.contains(key))
            .and_then(|root_val| {
                let root = nodes.remove(&root_val).unwrap();
                let mut queue = VecDeque::from([Rc::clone(&root)]);

                loop {
                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        let mut node = node.borrow_mut();
                        let node = &mut *node;

                        for child in [&mut node.left, &mut node.right].into_iter().flatten() {
                            let child_val = child.borrow().val;

                            if let Some(real_child) = nodes.remove(&child_val) {
                                drop(mem::replace(child, Rc::clone(&real_child)));
                                queue.push_back(real_child);
                            }
                        }
                    }

                    if queue.is_empty() {
                        break;
                    }
                }

                if !nodes.is_empty() {
                    return None;
                }

                Self::validate_binary_tree(i32::MIN, &root).map(|_| root)
            })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::can_merge(trees)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
