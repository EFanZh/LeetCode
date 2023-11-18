pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::{Entry, OccupiedEntry};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[derive(Default)]
struct Counter(HashMap<u32, u16>);

impl Counter {
    fn add(&mut self, key: u32) {
        self.0.entry(key).and_modify(|count| *count += 1).or_insert(1);
    }

    fn unwrap_occupied<K, V>(entry: Entry<K, V>) -> OccupiedEntry<K, V> {
        match entry {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => unreachable!(),
        }
    }

    fn remove(&mut self, key: u32) {
        let entry = Self::unwrap_occupied(self.0.entry(key));

        if *entry.get() == 1 {
            entry.remove();
        } else {
            *entry.into_mut() -= 1;
        }
    }

    fn query(&self, key: u32) -> u16 {
        self.0.get(&key).copied().unwrap_or(0)
    }
}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n_minus_1 = n as u32 - 1;

        let mut lamps = lamps
            .iter()
            .map(|lamp| {
                let [y, x]: [_; 2] = lamp.as_slice().try_into().unwrap();

                (y as u32, x as u32)
            })
            .collect::<HashSet<_>>();

        let mut row_cache = Counter::default();
        let mut column_cache = Counter::default();
        let mut diagonal_cache_1 = Counter::default();
        let mut diagonal_cache_2 = Counter::default();
        let indices = |y, x| (y, x, y + x, y + (n_minus_1 - x));

        for &(y, x) in &lamps {
            let (row_index, column_index, diagonal_index_1, diagonal_index_2) = indices(y, x);

            row_cache.add(row_index);
            column_cache.add(column_index);
            diagonal_cache_1.add(diagonal_index_1);
            diagonal_cache_2.add(diagonal_index_2);
        }

        queries
            .iter()
            .map(|query| {
                let [y, x]: [_; 2] = query.as_slice().try_into().unwrap();
                let y = y as u32;
                let x = x as u32;
                let (row_index, column_index, diagonal_index_1, diagonal_index_2) = indices(y, x);

                let result = i32::from(
                    row_cache.query(row_index)
                        | column_cache.query(column_index)
                        | diagonal_cache_1.query(diagonal_index_1)
                        | diagonal_cache_2.query(diagonal_index_2)
                        != 0,
                );

                for y in [y.wrapping_sub(1), y, y + 1] {
                    for x in [x.wrapping_sub(1), x, x + 1] {
                        if lamps.remove(&(y, x)) {
                            let (row_index, column_index, diagonal_index_1, diagonal_index_2) = indices(y, x);

                            row_cache.remove(row_index);
                            column_cache.remove(column_index);
                            diagonal_cache_1.remove(diagonal_index_1);
                            diagonal_cache_2.remove(diagonal_index_2);
                        }
                    }
                }

                result
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::grid_illumination(n, lamps, queries)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_unwrap_occupied_success() {
        super::Counter::unwrap_occupied(HashMap::from([(2_u32, 3_u16)]).entry(2));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_unwrap_occupied_failure() {
        super::Counter::unwrap_occupied(HashMap::from([(2_u32, 5_u16)]).entry(5));
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
