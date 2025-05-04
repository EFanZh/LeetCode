use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let k = k as u32 as usize;
        let mut queue = VecDeque::from_iter(root);
        let mut sums = Vec::new();

        while !queue.is_empty() {
            let mut sum = 0;

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                let (val, left, right) = {
                    let node_ref = node.borrow();

                    (node_ref.val, node_ref.left.clone(), node_ref.right.clone())
                };

                sum += u64::from(val as u32);

                queue.extend(left.into_iter().chain(right));
            }

            sums.push(sum);
        }

        let rows = sums.len();

        if rows < k {
            -1
        } else {
            *sums.select_nth_unstable(rows - k).1 as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        Self::kth_largest_level_sum(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
