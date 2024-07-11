pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    units: u16,
    count: u16,
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
        self.units.cmp(&other.units)
    }
}

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut queue = box_types
            .into_iter()
            .map(|box_type| {
                let [count, units]: [_; 2] = box_type.try_into().ok().unwrap();

                Item {
                    count: count as _,
                    units: units as _,
                }
            })
            .collect::<BinaryHeap<_>>();

        let mut available = truck_size as u32;
        let mut result = 0;

        while let Some(item) = queue.pop() {
            if u32::from(item.count) < available {
                result += u32::from(item.units) * u32::from(item.count);
                available -= u32::from(item.count);
            } else {
                result += u32::from(item.units) * available;

                break;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        Self::maximum_units(box_types, truck_size)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { units: 2, count: 3 } == Item { units: 2, count: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
