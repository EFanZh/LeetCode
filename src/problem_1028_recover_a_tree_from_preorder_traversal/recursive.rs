use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

type Item = (u32, i32);

impl Solution {
    fn helper(input: &[Item], depth: u32) -> (Option<Rc<RefCell<TreeNode>>>, &[Item]) {
        if let Some((&(first_depth, val), rest)) = input.split_first() {
            if first_depth == depth {
                let (left, rest) = Self::helper(rest, depth + 1);
                let (right, rest) = Self::helper(rest, depth + 1);

                return (Some(Rc::new(RefCell::new(TreeNode { val, left, right }))), rest);
            }
        }

        (None, input)
    }

    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut items = Vec::new();
        let mut traversal = traversal.as_bytes();

        while let Some(depth) = traversal.iter().position(|&c| c != b'-') {
            traversal = &traversal[depth..];

            let value_str_length = traversal.iter().position(|&c| c == b'-').unwrap_or(traversal.len());
            let (left, right) = traversal.split_at(value_str_length);
            let mut value = 0;

            for &c in left {
                value = value * 10 + i32::from(c - b'0');
            }

            items.push((depth as _, value));

            traversal = right;
        }

        Self::helper(&items, 0).0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recover_from_preorder(traversal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
