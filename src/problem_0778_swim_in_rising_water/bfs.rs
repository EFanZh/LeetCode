pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;

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
        let mut grid = grid;
        let n = grid.len();
        let mut result = mem::replace(&mut grid[0][0], -1);

        if n > 1 {
            let mut node = Item {
                value: result,
                y: 0,
                x: 0,
            };

            let mut queue = BinaryHeap::new();

            loop {
                for &(next_y, next_x) in &Self::neighbors(node.y, node.x) {
                    match grid
                        .get_mut(usize::from(next_y))
                        .and_then(|row| row.get_mut(usize::from(next_x)))
                    {
                        None | Some(-1) => {}
                        Some(value) => {
                            queue.push(Item {
                                value: mem::replace(value, -1),
                                y: next_y,
                                x: next_x,
                            });
                        }
                    }
                }

                node = queue.pop().unwrap();
                result = result.max(node.value);

                if (usize::from(node.y), usize::from(node.x)) == (n - 1, n - 1) {
                    break;
                }
            }
        }

        result
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
