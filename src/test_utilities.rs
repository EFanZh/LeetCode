use super::data_structures::{ListNode, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct UniqueTreeNode {
    val: i32,
    left: Option<Box<UniqueTreeNode>>,
    right: Option<Box<UniqueTreeNode>>,
}

fn make_unique_tree<I: IntoIterator<Item = Option<i32>>>(values: I) -> Option<Box<UniqueTreeNode>> {
    let mut result = None;
    let mut queue = VecDeque::new();
    let mut current_node_ref = &mut result;

    for maybe_value in values {
        if let Some(value) = maybe_value {
            *current_node_ref = Some(Box::new(UniqueTreeNode {
                val: value,
                left: None,
                right: None,
            }));

            let (left_ref, right_ref) = current_node_ref
                .as_mut()
                .map(|node| (&mut node.left, &mut node.right))
                .unwrap();

            queue.push_back(left_ref);
            queue.push_back(right_ref);
        }

        if let Some(new_current_node_ref) = queue.pop_front() {
            current_node_ref = new_current_node_ref;
        } else {
            break;
        }
    }

    result
}

fn unique_tree_to_tree(node: Option<Box<UniqueTreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    node.map(|real_node| {
        let mut node = TreeNode::new(real_node.val);

        node.left = unique_tree_to_tree(real_node.left);
        node.right = unique_tree_to_tree(real_node.right);

        Rc::new(RefCell::new(node))
    })
}

pub fn make_tree<I: IntoIterator<Item = Option<i32>>>(values: I) -> Option<Rc<RefCell<TreeNode>>> {
    let unique_treee = make_unique_tree(values);

    unique_tree_to_tree(unique_treee)
}

pub fn make_list<I: IntoIterator<Item = i32>>(values: I) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut node_ref = &mut result;

    for value in values {
        *node_ref = Some(Box::new(ListNode::new(value)));

        node_ref = &mut node_ref.as_mut().unwrap().next;
    }

    result
}
