pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let columns = grid.first().map_or(0, Vec::len);
        let mut cache = vec![(0, 0); columns];

        let mut grid_iter = grid.iter().enumerate().map(|(y, row)| {
            let next_row = grid.get(y + 1).map(Vec::as_slice);

            row.iter().enumerate().map(move |(x, &value)| {
                let bottom_cells = next_row.map_or((true, true), |next_row| {
                    (next_row[x] != 0, next_row.get(x + 1).copied() != Some(0))
                });

                (
                    value != 0,
                    row.get(x + 1).copied() != Some(0),
                    bottom_cells.0,
                    bottom_cells.1,
                )
            })
        });

        // Process first row;

        let mut first_row = grid_iter.next().unwrap();
        let (_, _, bottom, bottom_right) = first_row.next().unwrap();
        let mut prev = (0, if bottom || bottom_right { u16::MAX } else { 1 });

        cache[0] = prev;

        for (target, (current, right, bottom, bottom_right)) in cache[1..].iter_mut().zip(first_row) {
            *target = if current {
                (u16::MAX, u16::MAX)
            } else if right {
                (u16::MAX, if bottom { u16::MAX } else { prev.1.saturating_add(1) })
            } else if bottom {
                (prev.0.saturating_add(1), u16::MAX)
            } else {
                let candidate = (prev.0.saturating_add(1), prev.1.saturating_add(1));

                if bottom_right {
                    candidate
                } else {
                    (candidate.0, candidate.1.min(candidate.0.saturating_add(1)))
                }
            };

            prev = *target;
        }

        // Process rest rows.

        for row in grid_iter {
            let mut has_progress = false;
            let mut prev = (u16::MAX, u16::MAX);

            for (target, (current, right, bottom, bottom_right)) in cache.iter_mut().zip(row) {
                *target = if current {
                    (u16::MAX, u16::MAX)
                } else if right {
                    (
                        u16::MAX,
                        if bottom {
                            u16::MAX
                        } else {
                            prev.1.min(target.1).saturating_add(1)
                        },
                    )
                } else if bottom {
                    (prev.0.min(target.0).saturating_add(1), u16::MAX)
                } else {
                    let candidate = (
                        prev.0.min(target.0).saturating_add(1),
                        prev.1.min(target.1).saturating_add(1),
                    );

                    if bottom_right {
                        candidate
                    } else {
                        (
                            candidate.0.min(candidate.1.saturating_add(1)),
                            candidate.1.min(candidate.0.saturating_add(1)),
                        )
                    }
                };

                has_progress |= *target != (u16::MAX, u16::MAX);
                prev = *target;
            }

            if !has_progress {
                return -1;
            }
        }

        i32::from(cache[cache.len() - 2].0 as i16)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        Self::minimum_moves(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
