pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as u32 as usize;
        let n = n as u32 as usize;
        let mut grid = vec![0_u8; n * m].into_boxed_slice();

        for (c, positions) in [(b'g', guards), (b'w', walls)] {
            for position in positions {
                let [y, x] = <[_; 2]>::map(position.try_into().ok().unwrap(), |x| x as u32 as usize);

                grid[n * y + x] = c;
            }
        }

        let mut guarded = vec![false; n].into_boxed_slice();

        grid.chunks_exact_mut(n).for_each(|row| {
            let mut left_guarded = false;

            row.iter_mut().zip(&mut *guarded).for_each(|(cell, top_guarded)| {
                if *cell == 0 {
                    *cell = u8::from(left_guarded || *top_guarded);
                } else {
                    let guarded = *cell == b'g';

                    left_guarded = guarded;
                    *top_guarded = guarded;
                }
            });
        });

        guarded.fill(false);

        let mut result = 0;

        grid.chunks_exact(n).rev().for_each(|row| {
            let mut right_guarded = false;

            row.iter()
                .zip(&mut *guarded)
                .rev()
                .for_each(|(&cell, bottom_guarded)| match cell {
                    0 => result += i32::from(!(right_guarded || *bottom_guarded)),
                    1 => {}
                    _ => {
                        let guarded = cell == b'g';

                        right_guarded = guarded;
                        *bottom_guarded = guarded;
                    }
                });
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        Self::count_unguarded(m, n, guards, walls)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
