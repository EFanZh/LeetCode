use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(
        pre_iter: &mut impl Iterator<Item = i32>,
        post_first: i32,
        post_iter: &mut impl Iterator<Item = i32>,
    ) -> Rc<RefCell<TreeNode>> {
        let val = pre_iter.next().unwrap();

        let (left, right) = if post_first == val {
            (None, None)
        } else {
            let left = Self::helper(pre_iter, post_first, post_iter);
            let next_post_first = post_iter.next().unwrap();

            if next_post_first == val {
                (Some(left), None)
            } else {
                let right = Self::helper(pre_iter, next_post_first, post_iter);

                post_iter.next();

                (Some(left), Some(right))
            }
        };

        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }

    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_iter = preorder.iter().copied();
        let mut post_iter = postorder.iter().copied();

        post_iter
            .next()
            .map(|post_first| Self::helper(&mut pre_iter, post_first, &mut post_iter))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_from_pre_post(preorder, postorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
