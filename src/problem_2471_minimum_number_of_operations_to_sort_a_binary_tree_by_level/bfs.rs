use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = Vec::from_iter(root);
        let mut next_queue = Vec::new();
        let mut buffer = Vec::new();
        let mut result = 0;

        loop {
            #[expect(clippy::iter_with_drain, reason = "false positive")]
            queue.drain(..).for_each(|node| {
                let node = &*node.borrow();

                if let Some(left) = node.left.as_ref() {
                    next_queue.push(Rc::clone(left));
                }

                if let Some(right) = node.right.as_ref() {
                    next_queue.push(Rc::clone(right));
                }
            });

            if next_queue.is_empty() {
                break;
            }

            mem::swap(&mut queue, &mut next_queue);

            buffer.extend(queue.iter().map(|node| node.borrow().val).zip(0_u32..));
            buffer.sort_unstable_by_key(|&(value, _)| value);

            for start_index in 0..buffer.len() {
                let mut index = buffer[start_index].1;

                if start_index as u32 != index {
                    loop {
                        index = mem::replace(&mut buffer[index as usize].1, index);
                        result += 1;

                        if index == start_index as u32 {
                            break;
                        }
                    }
                }
            }

            buffer.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::minimum_operations(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
