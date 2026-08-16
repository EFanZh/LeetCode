use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_unwrapped = Rc::clone(root.as_ref().unwrap());

        root_unwrapped.borrow_mut().val = 0;

        let mut queue = VecDeque::from([root_unwrapped]);
        let mut sums = Vec::new();

        loop {
            let mut total_sum = 0;

            sums.extend(queue.iter().map(|node| {
                let node = &*node.borrow();

                let child_sum = [&node.left, &node.right]
                    .iter()
                    .filter_map(|child| child.as_deref().map(|child| child.borrow().val))
                    .sum::<i32>();

                total_sum += child_sum;

                child_sum
            }));

            for &child_sum in &sums {
                let target_value = total_sum - child_sum;
                let node = queue.pop_front().unwrap();
                let node = &*node.borrow();

                for child in [&node.left, &node.right].into_iter().flatten() {
                    child.borrow_mut().val = target_value;
                    queue.push_back(Rc::clone(child));
                }
            }

            if queue.is_empty() {
                break;
            }

            sums.clear();
        }

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::replace_value_in_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
