pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::convert::TryInto;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut operations = Vec::with_capacity(buildings.len() * 2);

        for building in buildings {
            let [left, right, height]: [_; 3] = building.as_slice().try_into().unwrap();

            operations.push((left, -height));
            operations.push((right, height));
        }

        operations.sort_unstable();

        let mut result = Vec::new();
        let mut heights = BTreeMap::new();
        let mut current_height = 0;

        for (x, h) in operations {
            if h < 0 {
                heights.entry(-h).and_modify(|count| *count += 1).or_insert(1);
            } else if let Entry::Occupied(entry) = heights.entry(h) {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }
            }

            let new_height = heights.keys().last().copied().unwrap_or(0);

            if new_height != current_height {
                current_height = new_height;

                result.push(vec![x, current_height]);
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
