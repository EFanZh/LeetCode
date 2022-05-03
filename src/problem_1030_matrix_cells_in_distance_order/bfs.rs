pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let rows = rows as u32;
        let cols = cols as u32;
        let r_center = r_center as u32;
        let c_center = c_center as u32;
        let mut result = Vec::with_capacity(cols as usize * rows as usize);
        let mut queue = VecDeque::new();
        let mut node = (r_center, c_center);

        loop {
            let (r, c) = node;

            result.push(vec![r as _, c as _]);

            if r <= r_center {
                if c <= c_center && r != 0 {
                    queue.push_back((r - 1, c));
                }

                if c >= c_center && c + 1 != cols {
                    queue.push_back((r, c + 1));
                }
            }

            if r >= r_center {
                if c <= c_center && c != 0 {
                    queue.push_back((r, c - 1));
                }

                if c >= c_center && r + 1 != rows {
                    queue.push_back((r + 1, c));
                }
            }

            if let Some(next_node) = queue.pop_front() {
                node = next_node;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        Self::all_cells_dist_order(rows, cols, r_center, c_center)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
