pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;
use std::num::NonZeroUsize;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().and_then(|row| NonZeroUsize::new(row.len())).unwrap();
        let mut new_grid = vec![false; columns.get() * rows];
        let mut i = 0;
        let mut box_location = 0;
        let mut player_location = 0;
        let mut target_location = 0;

        for row in grid {
            for c in row {
                match c {
                    '#' => new_grid[i] = true,
                    'B' => box_location = i,
                    'S' => player_location = i,
                    'T' => target_location = i,
                    _ => {}
                }

                i += 1;
            }
        }

        let mut queue_1 = vec![(box_location, player_location)];
        let mut queue_2 = Vec::new();
        let mut visited = HashSet::from([(box_location, player_location)]);
        let mut result = 1;

        loop {
            while let Some((box_location, player_location)) = queue_1.pop() {
                // Up and down.

                for offset in [usize::MIN.wrapping_sub(columns.get()), columns.get()] {
                    let next_player = player_location.wrapping_add(offset);

                    if new_grid.get(next_player).copied() == Some(false) {
                        if next_player == box_location {
                            let next_box = box_location.wrapping_add(offset);

                            if next_box == target_location {
                                return result;
                            }

                            if new_grid.get(next_box).copied() == Some(false) {
                                let next = (next_box, next_player);

                                if visited.insert(next) {
                                    queue_2.push(next);
                                }
                            }
                        } else {
                            let next = (box_location, next_player);

                            if visited.insert(next) {
                                queue_1.push(next);
                            }
                        }
                    }
                }

                // Left and right.

                let player_x = player_location % columns;
                let box_x = box_location % columns;

                for offset in [usize::MAX, 1] {
                    let next_player = player_location.wrapping_add(offset);

                    if player_x.wrapping_add(offset) < columns.get() && !new_grid[next_player] {
                        if next_player == box_location {
                            let next_box = box_location.wrapping_add(offset);

                            if box_x.wrapping_add(offset) < columns.get() {
                                if next_box == target_location {
                                    return result;
                                }

                                if !new_grid[next_box] {
                                    let next = (next_box, next_player);

                                    if visited.insert(next) {
                                        queue_2.push(next);
                                    }
                                }
                            }
                        } else {
                            let next = (box_location, next_player);

                            if visited.insert(next) {
                                queue_1.push(next);
                            }
                        }
                    }
                }
            }

            if queue_2.is_empty() {
                return -1;
            }

            mem::swap(&mut queue_1, &mut queue_2);

            result += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        Self::min_push_box(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
