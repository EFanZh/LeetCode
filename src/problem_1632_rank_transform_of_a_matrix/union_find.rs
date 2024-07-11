pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

// See <https://leetcode.com/problems/rank-transform-of-a-matrix/discuss/1391380/C%2B%2BPython-HashMap-and-Sorting-and-Union-Find-Clean-and-Concise>.

impl Solution {
    fn find_root(union_find: &mut [(u16, u16)], node: u16) -> (u16, u16) {
        let state = union_find[usize::from(node)];

        if state.0 == 0 {
            (node, state.1)
        } else {
            let result = Self::find_root(union_find, state.0 - 1);

            if state.0 != result.0 + 1 {
                union_find[usize::from(node)].0 = result.0 + 1;
            }

            result
        }
    }

    fn union(union_find: &mut [(u16, u16)], left: u16, right: u16) {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if left_root.0 != right_root.0 {
            let (child, parent) = if left_root.1 < right_root.1 {
                (left_root.0, right_root.0)
            } else {
                if left_root.1 == right_root.1 {
                    union_find[usize::from(left_root.0)].1 += 1;
                }

                (right_root.0, left_root.0)
            };

            union_find[usize::from(child)].0 = parent + 1;
        }
    }

    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        let rows = matrix.len();
        let columns = matrix.first().map_or(0, Vec::len);

        // Group cells by values.

        let mut cells_by_values = HashMap::<_, Vec<(u16, u16)>>::new();

        for (y, row) in (0..).zip(&matrix) {
            for (x, &value) in (0..).zip(row) {
                cells_by_values
                    .entry(value)
                    .and_modify(|coordinates| coordinates.push((y, x)))
                    .or_insert_with(|| vec![(y, x)]);
            }
        }

        // Sort cells by values.

        let mut sorted_cells = Vec::from_iter(cells_by_values);

        sorted_cells.sort_unstable_by_key(|&(key, _)| key);

        // Assign ranks.

        let column_start = rows as u16;
        let line_count = rows + columns;
        let mut ranks = vec![0; line_count].into_boxed_slice();
        let mut union_find = vec![(0, 0); line_count].into_boxed_slice();
        let mut max_ranks = vec![0; line_count].into_boxed_slice();

        for (_, cells) in sorted_cells {
            // Group lines.

            for &(y, x) in &cells {
                Self::union(&mut union_find, y, column_start + x);
            }

            // Find maximum rank of each group.

            for &(y, x) in &cells {
                let max_rank = &mut max_ranks[usize::from(Self::find_root(&mut union_find, y).0)];

                for line in [y, column_start + x] {
                    *max_rank = (*max_rank).max(ranks[usize::from(line)]);
                }
            }

            // Assign new ranks.

            for &(y, x) in &cells {
                let new_rank = max_ranks[usize::from(Self::find_root(&mut union_find, y).0)] + 1;

                for line in [y, column_start + x] {
                    ranks[usize::from(line)] = new_rank;
                }
            }

            // Update matrix.

            for &(y, x) in &cells {
                matrix[usize::from(y)][usize::from(x)] = ranks[usize::from(y)];
            }

            // Reset caches.

            for &(y, x) in &cells {
                for line in [y, column_start + x] {
                    let line = usize::from(line);

                    union_find[line] = (0, 0);
                    max_ranks[line] = 0;
                }
            }
        }

        matrix
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::matrix_rank_transform(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
