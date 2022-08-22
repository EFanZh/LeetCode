pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

struct Item {
    value: u32,
    label: u32,
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
        Ord::cmp(&self.value, &other.value)
    }
}

impl Solution {
    pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
        let use_limit = use_limit as u32;

        let mut queue = values
            .into_iter()
            .zip(labels)
            .map(|(value, label)| Item {
                value: value as _,
                label: label as _,
            })
            .collect::<BinaryHeap<_>>();

        let mut score = 0;
        let mut label_used = HashMap::new();
        let mut total_used = 0;

        while let Some(item) = queue.pop() {
            match label_used.entry(item.label) {
                Entry::Occupied(entry) => {
                    if *entry.get() < use_limit {
                        *entry.into_mut() += 1;
                    } else {
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }

            score += item.value;
            total_used += 1;

            if total_used == num_wanted {
                break;
            }
        }

        score as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
        Self::largest_vals_from_labels(values, labels, num_wanted, use_limit)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { value: 2, label: 3 } == Item { value: 2, label: 4 });
        assert!(Item { value: 2, label: 3 } != Item { value: 4, label: 3 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
