pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::mem;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut rains = rains;
        let mut prev_rainy_days = HashMap::<i32, usize>::new();
        let mut candidates = BTreeSet::new();
        let mut i = 0;

        while let Some(slot) = rains.get_mut(i) {
            if *slot == 0 {
                candidates.insert(i);
            } else {
                let lake = mem::replace(slot, -1);

                match prev_rainy_days.entry(lake) {
                    Entry::Occupied(entry) => {
                        let prev_rainy_day = entry.into_mut();

                        if let Some(&candidate) = candidates.range(*prev_rainy_day..).next() {
                            candidates.remove(&candidate);

                            rains[candidate] = lake;

                            *prev_rainy_day = i;
                        } else {
                            rains.clear();

                            return rains;
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(i);
                    }
                }
            }

            i += 1;
        }

        for day in candidates {
            rains[day] = 1;
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
