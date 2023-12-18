pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn insert(values: &mut HashMap<i32, u32>, value: i32) {
        values.entry(value).and_modify(|count| *count += 1).or_insert(1);
    }

    fn remove(values: &mut HashMap<i32, u32>, value: i32) -> bool {
        if let Entry::Occupied(entry) = values.entry(value) {
            if *entry.get() == 1 {
                entry.remove();
            } else {
                *entry.into_mut() -= 1;
            }

            true
        } else {
            false
        }
    }

    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;

        changed.sort_unstable();

        let mut values = HashMap::<i32, u32>::new();

        for &value in &changed {
            Self::insert(&mut values, value);
        }

        let mut retained = 0;
        let mut i = 0;

        while let Some(&x) = changed.get(i) {
            i += 1;

            if Self::remove(&mut values, x) {
                if Self::remove(&mut values, x * 2) {
                    changed[retained] = x;
                    retained += 1;
                } else {
                    retained = 0;

                    break;
                }
            }
        }

        changed.truncate(retained);

        changed
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        Self::find_original_array(changed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
