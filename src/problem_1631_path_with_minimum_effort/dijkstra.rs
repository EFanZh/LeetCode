pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    distance: u32,
    node: (u16, u16),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let rows = heights.len();
        let columns = heights.first().map_or(0, Vec::len);
        let target = (rows as u16 - 1, columns as u16 - 1);
        let mut distances = vec![u32::MAX; columns * rows].into_boxed_slice();

        distances[0] = 0;

        let mut item = Item {
            distance: 0,
            node: (0_u16, 0_u16),
        };

        let mut queue = BinaryHeap::new();

        loop {
            if item.node == target {
                return item.distance as _;
            }

            let node_height = heights[usize::from(item.node.0)][usize::from(item.node.1)] as u32;

            for neighbor in [
                (item.node.0.wrapping_sub(1), item.node.1),
                (item.node.0, item.node.1.wrapping_sub(1)),
                (item.node.0, item.node.1 + 1),
                (item.node.0 + 1, item.node.1),
            ] {
                if let Some(&neighbor_height) = heights
                    .get(usize::from(neighbor.0))
                    .and_then(|row| row.get(usize::from(neighbor.1)))
                {
                    let neighbor_height = neighbor_height as u32;

                    let height_diff = node_height.abs_diff(neighbor_height);

                    let candidate_distance = item.distance.max(height_diff);
                    let distance = &mut distances[columns * usize::from(neighbor.0) + usize::from(neighbor.1)];

                    if candidate_distance < *distance {
                        *distance = candidate_distance;

                        queue.push(Item {
                            distance: candidate_distance,
                            node: neighbor,
                        });
                    }
                }
            }

            loop {
                let next_item = queue.pop().unwrap();

                if next_item.distance
                    == distances[columns * usize::from(next_item.node.0) + usize::from(next_item.node.1)]
                {
                    item = next_item;

                    break;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        Self::minimum_effort_path(heights)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                distance: 2,
                node: (3, 5),
            } == Item {
                distance: 2,
                node: (7, 11),
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
