use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as u32;

        if n % 2 == 0 {
            Vec::new()
        } else {
            let n = (n / 2 + 1) as usize;
            let mut cache = Vec::with_capacity(n);

            cache.push(vec![Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })))]);

            // <https://oeis.org/A000108>.

            let mut catalan = 1;

            for i in 1..n {
                catalan = 2 * (2 * i - 1) * catalan / (i + 1);

                let mut trees = Vec::with_capacity(catalan);
                let children = &cache[..i];

                for (left, right) in children.iter().zip(children.iter().rev()) {
                    for left in left {
                        for right in right {
                            trees.push(Some(Rc::new(RefCell::new(TreeNode {
                                val: 0,
                                left: left.clone(),
                                right: right.clone(),
                            }))));
                        }
                    }
                }

                cache.push(trees);
            }

            cache.pop().unwrap()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::all_possible_fbt(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
