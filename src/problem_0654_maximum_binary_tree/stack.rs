use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::<Rc<RefCell<TreeNode>>>::with_capacity(nums.len());

        for val in nums {
            let mut left = None;

            let node = loop {
                if let Some(top) = stack.last() {
                    let mut top_ref = top.borrow_mut();

                    if top_ref.val < val {
                        drop(top_ref);

                        left = stack.pop();
                    } else {
                        let node = Rc::new(RefCell::new(TreeNode { val, left, right: None }));

                        top_ref.right = Some(Rc::clone(&node));

                        break node;
                    }
                } else {
                    break Rc::new(RefCell::new(TreeNode { val, left, right: None }));
                }
            };

            stack.push(node);
        }

        stack.into_iter().next()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_maximum_binary_tree(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
