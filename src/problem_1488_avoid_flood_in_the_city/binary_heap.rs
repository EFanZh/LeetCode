pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

struct Lake {
    id: i32,
    rainy_days: Vec<u32>,
    cursor: usize,
    is_full: bool,
}

struct Item {
    day: u32,
    alt_id: u32,
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
        other.day.cmp(&self.day)
    }
}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut rains = rains;
        let mut lake_to_id = HashMap::<i32, u32>::new();
        let mut id_to_lake = Vec::<Lake>::new();

        for (i, slot) in (0..).zip(&mut rains) {
            *slot = if *slot == 0 {
                -1
            } else {
                match lake_to_id.entry(*slot) {
                    Entry::Occupied(entry) => {
                        let alt_id = *entry.get();

                        id_to_lake[alt_id as usize].rainy_days.push(i);

                        alt_id as _
                    }
                    Entry::Vacant(entry) => {
                        let alt_id = id_to_lake.len() as _;

                        entry.insert(alt_id);

                        id_to_lake.push(Lake {
                            id: *slot,
                            rainy_days: vec![i],
                            cursor: 0,
                            is_full: false,
                        });

                        alt_id as _
                    }
                }
            };
        }

        drop(lake_to_id);

        let mut full_lakes = BinaryHeap::<Item>::new();

        for slot in &mut rains {
            *slot = if *slot == -1 {
                full_lakes.pop().map_or(1, |item| {
                    let lake = &mut id_to_lake[item.alt_id as usize];

                    lake.is_full = false;

                    lake.id
                })
            } else {
                let alt_id = *slot as u32;
                let lake = &mut id_to_lake[alt_id as usize];

                if lake.is_full {
                    rains.clear();

                    break;
                }

                lake.cursor += 1;
                lake.is_full = true;

                if let Some(&lake_day) = lake.rainy_days.get(lake.cursor) {
                    full_lakes.push(Item { day: lake_day, alt_id });
                }

                -1
            };
        }

        rains
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        Self::avoid_flood(rains)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { day: 2, alt_id: 3 } == Item { day: 2, alt_id: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
