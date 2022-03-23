use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    fn new_cell(row: u16, value: i32) -> HashMap<u16, Vec<i32>> {
        HashMap::from([(row, vec![value])])
    }

    fn helper(
        node: Option<&RefCell<TreeNode>>,
        row: u16,
        column: i16,
        cells: &mut VecDeque<HashMap<u16, Vec<i32>>>,
        start_column: &mut i16,
    ) {
        if let Some(node) = node.map(RefCell::borrow) {
            let index = (column - *start_column) as usize;

            if let Some(column) = cells.get_mut(index) {
                column
                    .entry(row)
                    .and_modify(|values| values.push(node.val))
                    .or_insert_with(|| vec![node.val]);
            } else if index == cells.len() {
                cells.push_back(Self::new_cell(row, node.val));
            } else {
                cells.push_front(Self::new_cell(row, node.val));

                *start_column -= 1;
            }

            Self::helper(node.left.as_deref(), row + 1, column - 1, cells, start_column);
            Self::helper(node.right.as_deref(), row + 1, column + 1, cells, start_column);
        }
    }

    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cells = VecDeque::new();

        Self::helper(root.as_deref(), 0, 0, &mut cells, &mut 0);

        cells
            .into_iter()
            .map(|column| {
                let mut rows = column.into_iter().collect::<Vec<_>>();

                rows.sort_unstable_by_key(|&(row, _)| row);

                rows.into_iter()
                    .flat_map(|(_, mut cells)| {
                        cells.sort_unstable();

                        cells
                    })
                    .collect()
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::vertical_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
