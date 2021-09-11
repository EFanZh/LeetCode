use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

enum FindResult {
    NotFound,
    Found,
    Index(usize),
}

impl Solution {
    fn build_map(node: Option<&RefCell<TreeNode>>, map: &mut HashMap<(i32, usize, usize), usize>) -> usize {
        node.map_or(0, |node| {
            let node = node.borrow();
            let left_index = Self::build_map(node.left.as_deref(), map);
            let right_index = Self::build_map(node.right.as_deref(), map);
            let candidate_index = map.len() + 1;

            *map.entry((node.val, left_index, right_index))
                .or_insert(candidate_index)
        })
    }

    fn find_subtree(
        node: Option<&RefCell<TreeNode>>,
        map: &HashMap<(i32, usize, usize), usize>,
        target: usize,
    ) -> FindResult {
        node.map_or(FindResult::Index(0), |node| {
            let node = node.borrow();

            match Self::find_subtree(node.left.as_deref(), map, target) {
                FindResult::NotFound => match Self::find_subtree(node.right.as_deref(), map, target) {
                    FindResult::NotFound | FindResult::Index(_) => FindResult::NotFound,
                    FindResult::Found => FindResult::Found,
                },
                FindResult::Found => FindResult::Found,
                FindResult::Index(left_index) => match Self::find_subtree(node.right.as_deref(), map, target) {
                    FindResult::NotFound => FindResult::NotFound,
                    FindResult::Found => FindResult::Found,
                    FindResult::Index(right_index) => {
                        map.get(&(node.val, left_index, right_index))
                            .map_or(FindResult::NotFound, |&index| {
                                if index == target {
                                    FindResult::Found
                                } else {
                                    FindResult::Index(index)
                                }
                            })
                    }
                },
            }
        })
    }

    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut map = HashMap::new();
        let target = Self::build_map(t.as_deref(), &mut map);

        matches!(Self::find_subtree(s.as_deref(), &map, target), FindResult::Found)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_subtree(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
