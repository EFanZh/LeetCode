pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::Infallible;
use std::ops::ControlFlow;

const TURN_BIT: u16 = 1 << 12;
const CAT_TURN_BIT: u16 = 0;
const MOUSE_TURN_BIT: u16 = TURN_BIT;

const WIN_BIT: u16 = 1 << 13;
const CAT_WIN_BIT: u16 = 0;
const MOUSE_WIN_BIT: u16 = WIN_BIT;

struct Context {
    grid: [u8; 64],
    rows: u8,
    columns: u8,
    cat_jump: u8,
    mouse_jump: u8,
}

impl Solution {
    fn normalize_grid(grid: Vec<String>) -> ([u8; 64], u8, u8) {
        let mut result_grid = [0; 64];
        let rows = grid.len() as u8;
        let columns = grid.first().map_or(0, String::len) as u8;

        for (target_row, row) in result_grid.chunks_exact_mut(8).zip(grid) {
            target_row[..row.len()].copy_from_slice(row.as_bytes());
        }

        (result_grid, rows, columns)
    }

    fn next_positions<B>(
        grid: &[u8; 64],
        rows: u8,
        columns: u8,
        max_jump: u8,
        y: u8,
        x: u8,
        mut f: impl FnMut(u8, u8) -> ControlFlow<B>,
    ) -> ControlFlow<B> {
        f(y, x)?;

        for step in [1, u8::MAX] {
            // Vertical jump.

            {
                let mut y = y;
                let mut jump = max_jump;

                loop {
                    y = y.wrapping_add(step);

                    if y < rows && grid[usize::from(8 * y + x)] != b'#' {
                        f(y, x)?;

                        jump -= 1;

                        if jump == 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            // Horizontal jump.

            {
                let mut x = x;
                let mut jump = max_jump;

                loop {
                    x = x.wrapping_add(step);

                    if x < columns && grid[usize::from(8 * y + x)] != b'#' {
                        f(y, x)?;

                        jump -= 1;

                        if jump == 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }

    fn get_in_degree(grid: &[u8; 64], rows: u8, columns: u8, max_jump: u8, y: u8, x: u8) -> u8 {
        let mut result = 0;

        Self::next_positions::<Infallible>(grid, rows, columns, max_jump, y, x, |_, _| {
            result += 1;

            ControlFlow::Continue(())
        });

        result
    }

    fn bfs(
        Context {
            grid,
            rows,
            columns,
            cat_jump,
            mouse_jump,
        }: Context,
        target_state: u16,
        in_degrees: &mut [u8; 1 << 13],
        mut queue: VecDeque<u16>,
    ) -> bool {
        while let Some(state) = queue.pop_front() {
            let outcome = state & WIN_BIT;
            let turn = state & TURN_BIT;
            let cat_y = (state >> 9) & 7;
            let cat_x = (state >> 6) & 7;
            let mouse_y = (state >> 3) & 7;
            let mouse_x = state & 7;

            let (max_jump, y, x, mut base_next_state, offset_bits) = if turn == CAT_TURN_BIT {
                (mouse_jump, mouse_y, mouse_x, (cat_y << 9) | (cat_x << 6), 0)
            } else {
                (cat_jump, cat_y, cat_x, (mouse_y << 3) | mouse_x, 6)
            };

            base_next_state |= turn ^ TURN_BIT;

            let result = if (outcome == CAT_WIN_BIT) == (turn == CAT_TURN_BIT) {
                Self::next_positions(&grid, rows, columns, max_jump, y as _, x as _, |y, x| {
                    let next_state = base_next_state | (((u16::from(y) << 3) | u16::from(x)) << offset_bits);
                    let in_degree = &mut in_degrees[usize::from(next_state)];

                    if *in_degree != 0 {
                        *in_degree -= 1;

                        if *in_degree == 0 {
                            if next_state == target_state {
                                return ControlFlow::Break(());
                            }

                            queue.push_back(outcome | next_state);
                        }
                    }

                    ControlFlow::Continue(())
                })
            } else {
                Self::next_positions(&grid, rows, columns, max_jump, y as _, x as _, |y, x| {
                    let next_state = base_next_state | (((u16::from(y) << 3) | u16::from(x)) << offset_bits);
                    let in_degree = &mut in_degrees[usize::from(next_state)];

                    if *in_degree != 0 {
                        if next_state == target_state {
                            return ControlFlow::Break(());
                        }

                        *in_degree = 0;

                        queue.push_back(outcome | next_state);
                    }

                    ControlFlow::Continue(())
                })
            };

            if result.is_break() {
                return outcome != CAT_WIN_BIT;
            }
        }

        false
    }

    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let (grid, rows, columns) = Self::normalize_grid(grid);
        let (cat_jump, mouse_jump) = (cat_jump as u8, mouse_jump as u8);

        // Find positions.

        let mut target_state = MOUSE_TURN_BIT;
        let mut food_position = 0;

        for y in 0..rows {
            for x in 0..columns {
                match grid[usize::from(8 * y + x)] {
                    b'#' => continue,
                    b'C' => target_state |= (u16::from(y) << 9) | (u16::from(x) << 6),
                    b'F' => food_position = 8 * y + x,
                    b'M' => target_state |= (u16::from(y) << 3) | u16::from(x),
                    _ => {}
                }
            }
        }

        // Calculate in-degrees and enqueue terminal states.

        let mut in_degrees = [0_u8; 1 << 13];
        let mut queue = VecDeque::new();

        for cat_y in 0..rows {
            let base_state = u16::from(cat_y) << 9;

            for cat_x in 0..columns {
                let cat_position = 8 * cat_y + cat_x;

                if grid[usize::from(cat_position)] != b'#' {
                    let base_state = base_state | (u16::from(cat_x) << 6);
                    let cat_in_degree = Self::get_in_degree(&grid, rows, columns, cat_jump, cat_y, cat_x);

                    for mouse_y in 0..rows {
                        let base_state = base_state | (u16::from(mouse_y) << 3);

                        for mouse_x in 0..columns {
                            let mouse_position = 8 * mouse_y + mouse_x;
                            let base_state = base_state | u16::from(mouse_x);

                            if cat_position == mouse_position {
                                if cat_position != food_position {
                                    // Cat wins.
                                    queue.extend([
                                        CAT_WIN_BIT | CAT_TURN_BIT | base_state,
                                        CAT_WIN_BIT | MOUSE_TURN_BIT | base_state,
                                    ]);
                                }
                            } else if grid[usize::from(mouse_position)] != b'#' {
                                if cat_position == food_position {
                                    // Cat wins.
                                    queue.push_back(CAT_WIN_BIT | MOUSE_TURN_BIT | base_state);
                                } else if mouse_position == food_position {
                                    // Mouse wins.
                                    queue.push_back(MOUSE_WIN_BIT | CAT_TURN_BIT | base_state);
                                } else {
                                    let mouse_in_degree =
                                        Self::get_in_degree(&grid, rows, columns, mouse_jump, mouse_y, mouse_x);

                                    // Cat’s turn.
                                    in_degrees[usize::from(CAT_TURN_BIT | base_state)] = cat_in_degree;

                                    // Mouse’s turn.
                                    in_degrees[usize::from(MOUSE_TURN_BIT | base_state)] = mouse_in_degree;
                                }
                            }
                        }
                    }
                }
            }
        }

        // BFS.

        Self::bfs(
            Context {
                grid,
                rows,
                columns,
                cat_jump,
                mouse_jump,
            },
            target_state,
            &mut in_degrees,
            queue,
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        Self::can_mouse_win(grid, cat_jump, mouse_jump)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
