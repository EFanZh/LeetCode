pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;
use std::convert::TryInto;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut operations = Vec::with_capacity(buildings.len() * 2);

        for building in buildings {
            let [left, right, height]: [_; 3] = building.as_slice().try_into().unwrap();

            operations.push((left, right, height));
            operations.push((right, 0, 0));
        }

        operations.sort_unstable_by_key(|&(position, _, height)| (position, Reverse(height)));

        let mut result = Vec::new();
        let mut heights = BinaryHeap::new();
        let mut current_height = 0;

        for (x, right, height) in operations {
            if height != 0 {
                heights.push((height, right));
            }

            let new_height = loop {
                if let Some(top) = heights.peek_mut() {
                    if top.1 <= x {
                        PeekMut::pop(top);
                    } else {
                        break top.0;
                    }
                } else {
                    break 0;
                }
            };

            if new_height != current_height {
                current_height = new_height;

                result.push(vec![x, new_height]);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::get_skyline(buildings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
