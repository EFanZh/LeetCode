pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context {
    graph: [u64; 60],
    visited: u64,
    path: u64,
}

impl Solution {
    fn check_cycle(context: &mut Context, node: u8) -> bool {
        let mut nexts = context.graph[usize::from(node)] & !(1 << node);

        while nexts != 0 {
            let next = nexts.trailing_zeros() as u8;
            let next_probe = 1 << next;

            if context.visited & next_probe == 0 {
                context.visited |= next_probe;
                context.path |= next_probe;

                if Self::check_cycle(context, next) {
                    return true;
                }
            } else if context.path & next_probe != 0 {
                return true;
            }

            context.path &= !next_probe;
            nexts &= nexts - 1;
        }

        false
    }

    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        // Calculate color bounds.

        let mut bounds = [(u8::MAX, u8::MAX, 0, 0); 60];

        for (y, row) in (0..).zip(&target_grid) {
            for (x, &color) in (0..).zip(row) {
                let bound = &mut bounds[color as u32 as usize - 1];

                *bound = (bound.0.min(y), bound.1.min(x), y, bound.3.max(x));
            }
        }

        let mut row_operations = [(0_u64, 0_u64); 61];
        let mut column_operations = [(0_u64, 0_u64); 61];

        for (color, bound) in (0..).zip(bounds) {
            if bound.0 != u8::MAX {
                let probe = 1 << color;

                row_operations[usize::from(bound.0)].0 |= probe;
                row_operations[usize::from(bound.2) + 1].1 |= probe;
                column_operations[usize::from(bound.1)].0 |= probe;
                column_operations[usize::from(bound.3) + 1].1 |= probe;
            }
        }

        // Generate dependency graph.

        let mut context = Context {
            graph: [0; 60],
            visited: 0,
            path: 0,
        };

        let mut row_colors = 0;
        let mut max_color = 0;

        for (row, row_operation) in target_grid.iter().zip(row_operations) {
            row_colors |= row_operation.0;
            row_colors &= !row_operation.1;

            let mut column_colors = 0;

            for (&color, column_operation) in row.iter().zip(column_operations) {
                let color = color as u32 as usize - 1;

                column_colors |= column_operation.0;
                column_colors &= !column_operation.1;

                context.graph[color] |= row_colors & column_colors;
                max_color = max_color.max(color as u8);
            }
        }

        // Cycle detection.

        for node in 0..=max_color {
            let probe = 1 << node;

            if context.visited & probe == 0 {
                context.visited |= probe;
                context.path |= probe;

                if Self::check_cycle(&mut context, node) {
                    return false;
                }

                context.path &= !probe;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        Self::is_printable(target_grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
