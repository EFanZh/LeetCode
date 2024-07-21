pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::{mem, ptr};

type State = (Cell<u16>, Cell<u16>);

impl Solution {
    fn get_node(array: &[State], value: &State) -> u16 {
        ((ptr::from_ref(value) as usize - array.as_ptr() as usize) / mem::size_of::<State>()) as _
    }

    fn find_root<'a>(union_find: &'a [State], node_state: &'a State) -> &'a State {
        let parent = node_state.0.get();

        union_find.get(usize::from(parent)).map_or(node_state, |parent_state| {
            let root = Self::find_root(union_find, parent_state);

            node_state.0.set(Self::get_node(union_find, root));

            root
        })
    }

    fn union(union_find: &[State], left_state: &State, right_state: &State) {
        let left_root = Self::find_root(union_find, left_state);
        let right_root = Self::find_root(union_find, right_state);

        if !ptr::eq(left_root, right_root) {
            let left_rank = left_root.1.get();
            let right_rank = right_root.1.get();

            if left_rank < right_rank {
                left_root.0.set(Self::get_node(union_find, right_root));
            } else {
                if left_rank == right_rank {
                    left_root.1.set(left_rank + 1);
                }

                right_root.0.set(Self::get_node(union_find, left_root));
            }
        }
    }

    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        // Rotate the matrix.

        let (row, col) = (usize::from(col as u16), usize::from(row as u16));
        let stride = col + 2;
        let union_find = vec![(Cell::new(u16::MAX), Cell::new(u16::MAX)); stride * (row + 2)];

        // Fill top border.

        union_find.first().unwrap().1.set(1);

        for state in &union_find[1..stride] {
            state.0.set(0);
            state.1.set(0);
        }

        // Fill bottom border.

        let bottom_start = union_find.len() - stride;

        union_find[bottom_start].1.set(1);

        for state in &union_find[bottom_start + 1..] {
            state.0.set(bottom_start as _);
            state.1.set(0);
        }

        // Union neighboring cells.

        let start_state = union_find.first().unwrap();
        let end_state = union_find.last().unwrap();
        let mut day = 0;

        for cell in cells {
            let [x, y]: [_; 2] = cell.try_into().ok().unwrap();
            let node = stride * usize::from(y as u16) + usize::from(x as u16);
            let node_state = &union_find[node];

            node_state.1.set(0);

            for neighbor in [
                node - stride - 1,
                node - stride,
                node - stride + 1,
                node - 1,
                node + 1,
                node + stride - 1,
                node + stride,
                node + stride + 1,
            ] {
                let neighbor_state = &union_find[neighbor];

                if neighbor_state.1.get() != u16::MAX {
                    Self::union(&union_find, node_state, neighbor_state);
                }
            }

            if ptr::eq(
                Self::find_root(&union_find, start_state),
                Self::find_root(&union_find, end_state),
            ) {
                break;
            }

            day += 1;
        }

        day
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        Self::latest_day_to_cross(row, col, cells)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
