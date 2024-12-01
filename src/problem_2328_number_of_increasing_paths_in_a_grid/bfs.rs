pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    fn assign_add(target: &mut u32, value: u32) {
        *target += value;
        *target = target.checked_sub(1_000_000_007).unwrap_or(*target);
    }

    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut states = vec![0_u8; columns * rows].into_boxed_slice();
        let mut queue = VecDeque::new();

        grid.iter()
            .zip(states.chunks_exact_mut(columns))
            .enumerate()
            .for_each(|(y, (row, states))| {
                row.iter()
                    .zip(states)
                    .enumerate()
                    .for_each(|(x, (&value, target_state))| {
                        let mut state = 0;

                        for (neighbor, neighbor_y, neighbor_x) in [
                            (1 << 0, y.wrapping_sub(1), x),
                            (1 << 1, y, x.wrapping_sub(1)),
                            (1 << 2, y, x + 1),
                            (1 << 3, y + 1, x),
                        ] {
                            if let Some(&neighbor_value) = grid.get(neighbor_y).and_then(|row| row.get(neighbor_x)) {
                                match neighbor_value.cmp(&value) {
                                    Ordering::Less => state |= neighbor,
                                    Ordering::Equal => {}
                                    Ordering::Greater => state += 1 << 4,
                                }
                            }
                        }

                        *target_state = state;

                        if state < 0b_10000 {
                            queue.push_back(columns as u32 * y as u32 + x as u32);
                        }
                    });
            });

        drop(grid);

        let mut cache = vec![1_u32; columns * rows].into_boxed_slice();

        while let Some(node) = queue.pop_front() {
            let node = node as usize;
            let state = states[node];
            let count = cache[node];

            for (mask, offset) in [
                (1 << 0, columns.wrapping_neg()),
                (1 << 1, usize::MAX),
                (1 << 2, 1),
                (1 << 3, columns),
            ] {
                if state & mask != 0 {
                    let neighbor = node.wrapping_add(offset);
                    let neighbor_state = &mut states[neighbor];
                    let neighbor_count = &mut cache[neighbor];

                    *neighbor_state -= 1 << 4;

                    if *neighbor_state < 0b_10000 {
                        queue.push_back(neighbor as _);
                    }

                    Self::assign_add(neighbor_count, count);
                }
            }
        }

        let mut result = 0;

        for &count in &*cache {
            Self::assign_add(&mut result, count);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        Self::count_paths(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
