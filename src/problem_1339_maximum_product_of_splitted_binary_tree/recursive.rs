use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper<T>(node: Option<&RefCell<TreeNode>>, f: &mut impl FnMut(u32, T, T) -> T) -> T
    where
        T: Default,
    {
        if let Some(node) = node.map(RefCell::borrow) {
            let left_result = Self::helper(node.left.as_deref(), f);
            let right_result = Self::helper(node.right.as_deref(), f);

            f(node.val as _, left_result, right_result)
        } else {
            T::default()
        }
    }

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.as_deref();
        let mut root_sum = 0;

        Self::helper(root, &mut |value, (), ()| root_sum += value);

        let mut result = 0;

        Self::helper(root, &mut |value, left_sum: u32, right_sum: u32| {
            let sum = value + left_sum + right_sum;

            result = result.max(u64::from(sum) * u64::from(root_sum - sum));

            sum
        });

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_product(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
