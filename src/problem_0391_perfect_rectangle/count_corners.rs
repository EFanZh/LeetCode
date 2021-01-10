pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        const BOTTOM_LEFT_CORNER: u8 = 1;
        const BOTTOM_RIGHT_CORNER: u8 = 2;
        const TOP_LEFT_CORNER: u8 = 4;
        const TOP_RIGHT_CORNER: u8 = 8;
        const BOTTOM_SIDE: u8 = BOTTOM_LEFT_CORNER | BOTTOM_RIGHT_CORNER;
        const LEFT_SIDE: u8 = BOTTOM_LEFT_CORNER | TOP_LEFT_CORNER;
        const TOP_SIDE: u8 = TOP_LEFT_CORNER | TOP_RIGHT_CORNER;
        const RIGHT_SIDE: u8 = BOTTOM_RIGHT_CORNER | TOP_RIGHT_CORNER;

        let mut points = HashMap::new();

        for rectangle in rectangles {
            let [left, bottom, right, top]: [i32; 4] = rectangle.as_slice().try_into().unwrap();

            for &(point, corner) in &[
                ((left, bottom), (BOTTOM_LEFT_CORNER)),
                ((left, top), (TOP_LEFT_CORNER)),
                ((right, bottom), (BOTTOM_RIGHT_CORNER)),
                ((right, top), (TOP_RIGHT_CORNER)),
            ] {
                match points.entry(point) {
                    Entry::Occupied(entry) => {
                        if entry.get() & corner == 0 {
                            *entry.into_mut() |= corner;
                        } else {
                            return false;
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(corner);
                    }
                }
            }
        }

        let mut corners = 0;

        for &corner in points.values() {
            match corner {
                BOTTOM_LEFT_CORNER | BOTTOM_RIGHT_CORNER | TOP_LEFT_CORNER | TOP_RIGHT_CORNER => {
                    if corners & corner == 0 {
                        corners |= corner;
                    } else {
                        return false;
                    }
                }
                BOTTOM_SIDE | LEFT_SIDE | TOP_SIDE | RIGHT_SIDE => {}
                _ => return false,
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        Self::is_rectangle_cover(rectangles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
