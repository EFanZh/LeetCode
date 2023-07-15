pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Clock(u16);

impl Clock {
    fn now(&mut self) -> u16 {
        self.0 += 1;

        self.0
    }
}

struct Context {
    grid: [u32; 32],
    states: [u16; 32 * 32], // Discovery time.
    clock: Clock,
}

impl Solution {
    fn dfs(context: &mut Context, parent: (u8, u8), node: (u8, u8), discovery_time: u16) -> Option<u16> {
        let mut min_time = Some(discovery_time);

        for child in [
            (node.0 - 1, node.1),
            (node.0, node.1 - 1),
            (node.0, node.1 + 1),
            (node.0 + 1, node.1),
        ] {
            if child != parent && context.grid[usize::from(child.0)] & (1 << child.1) != 0 {
                let state = &mut context.states[32 * usize::from(child.0) + usize::from(child.1)];

                let child_min_time = if *state == 0 {
                    let child_discovery_time = context.clock.now();

                    *state = child_discovery_time;

                    Self::dfs(context, node, child, child_discovery_time)
                        .filter(|&child_min_time| child_min_time < discovery_time)
                } else {
                    Some(*state)
                };

                min_time = min_time.min(child_min_time);
            }
        }

        min_time
    }

    fn has_articulation_point(context: &mut Context, node: (u8, u8)) -> bool {
        // Check if the sub graph rooted at `(y, x)` contains articulation points.

        let mut seen_child = false;
        let mut result = false;

        for child in [(node.0, node.1 + 1), (node.0 + 1, node.1)] {
            if context.grid[usize::from(child.0)] & (1 << child.1) != 0 {
                let state = &mut context.states[32 * usize::from(child.0) + usize::from(child.1)];

                if *state == 0 {
                    if seen_child {
                        // Root node is an articulation point because it has more than one child.
                        result = true;
                    } else {
                        seen_child = true;
                    }

                    let child_discovery_time = context.clock.now();

                    *state = child_discovery_time;

                    // Child nodes contains articulation points.
                    result |= Self::dfs(context, node, child, child_discovery_time).is_none();
                }
            }
        }

        result
    }

    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as u8;
        let columns = grid.first().map_or(0, Vec::len) as u8;

        assert!(rows <= 30 && columns <= 30);

        // Build graph.

        let grid = {
            let mut cells = 0;
            let mut new_grid = [0_u32; 32];

            for (target_row, row) in new_grid[1..].iter_mut().zip(grid) {
                let mut new_row = 0;

                for (i, cell) in (1..).zip(row) {
                    let cell = cell as u32;

                    cells += cell;
                    new_row |= cell << i;
                }

                *target_row = new_row;
            }

            // Special case.

            if cells < 2 {
                return cells as _;
            }

            new_grid
        };

        // DFS.

        let mut seen_island = false;
        let mut has_articulation_point = false;

        let mut context = Context {
            grid,
            states: [0; 32 * 32],
            clock: Clock(0),
        };

        for y in 1..=rows {
            for x in 1..=columns {
                if grid[usize::from(y)] & (1 << x) != 0 {
                    let state = &mut context.states[32 * usize::from(y) + usize::from(x)];

                    if *state == 0 {
                        // We have found a new island.

                        if seen_island {
                            return 0; // We have seen two islands.
                        }

                        seen_island = true;

                        *state = context.clock.now();

                        has_articulation_point |= Self::has_articulation_point(&mut context, (y, x));
                    }
                }
            }
        }

        2 - i32::from(has_articulation_point)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_days(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
