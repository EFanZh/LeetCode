pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context<'a> {
    grid: &'a [Vec<char>],
    columns: usize,
    states: Box<[bool]>,
    group: char,
}

impl Solution {
    fn dfs(context: &mut Context, parent: (u16, u16), node: (u16, u16)) -> bool {
        for neighbor in [
            (node.0.wrapping_sub(1), node.1),
            (node.0, node.1.wrapping_sub(1)),
            (node.0, node.1 + 1),
            (node.0 + 1, node.1),
        ] {
            if neighbor != parent
                && let Some(&neighbor_char) = context
                    .grid
                    .get(usize::from(neighbor.0))
                    .and_then(|row| row.get(usize::from(neighbor.1)))
                && neighbor_char == context.group
            {
                let state = &mut context.states[context.columns * usize::from(neighbor.0) + usize::from(neighbor.1)];

                if *state {
                    return true;
                }

                *state = true;

                if Self::dfs(context, node, neighbor) {
                    return true;
                }
            }
        }

        false
    }

    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);

        let mut context = Context {
            grid: &grid,
            columns,
            states: vec![false; columns * rows].into_boxed_slice(),
            group: char::MAX,
        };

        for (y, row) in context.grid.iter().enumerate() {
            for (x, &group) in row.iter().enumerate() {
                if let state @ false = &mut context.states[context.columns * y + x] {
                    *state = true;

                    context.group = group;

                    if Self::dfs(&mut context, (u16::MAX, u16::MAX), (y as _, x as _)) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        Self::contains_cycle(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
