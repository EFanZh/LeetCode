pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    value: i32,
    y: u16,
    x: u16,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.value, &self.value)
    }
}

impl Solution {
    fn neighbors(y: u16, x: u16) -> [(u16, u16); 4] {
        [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)]
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let grid = grid;
        let n = grid.len();

        if n < 2 {
            grid[0][0]
        } else {
            let mut left = Item {
                value: grid[0][0],
                y: 0,
                x: 0,
            };

            let mut left_queue = BinaryHeap::new();

            let mut right = Item {
                value: grid[n - 1][n - 1],
                y: (n - 1) as _,
                x: (n - 1) as _,
            };

            let mut right_queue = BinaryHeap::new();

            let mut states = vec![0_u8; n * n];

            states[0] = 1;
            states[n * n - 1] = 2;

            loop {
                if right.value < left.value {
                    for &(next_y, next_x) in &Self::neighbors(right.y, right.x) {
                        if (next_y, next_x) == (left.y, left.x) {
                            return left.value;
                        }

                        if usize::from(next_y) < n && usize::from(next_x) < n {
                            let state = &mut states[n * usize::from(next_y) + usize::from(next_x)];

                            if *state & 2 == 0 {
                                *state |= 2;

                                right_queue.push(Item {
                                    value: grid[usize::from(next_y)][usize::from(next_x)],
                                    y: next_y,
                                    x: next_x,
                                });
                            }
                        }
                    }

                    right = right_queue.pop().unwrap();
                } else {
                    for &(next_y, next_x) in &Self::neighbors(left.y, left.x) {
                        if (next_y, next_x) == (right.y, right.x) {
                            return right.value;
                        }

                        if usize::from(next_y) < n && usize::from(next_x) < n {
                            let state = &mut states[n * usize::from(next_y) + usize::from(next_x)];

                            if *state & 1 == 0 {
                                *state |= 1;

                                left_queue.push(Item {
                                    value: grid[usize::from(next_y)][usize::from(next_x)],
                                    y: next_y,
                                    x: next_x,
                                });
                            }
                        }
                    }

                    left = left_queue.pop().unwrap();
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        Self::swim_in_water(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_item_eq() {
        assert!(Item { value: 2, y: 3, x: 5 } == Item { value: 2, y: 7, x: 9 });
    }
}
