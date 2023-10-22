pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

struct Item {
    count: u32,
    count_count: u32,
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
        self.count.cmp(&other.count)
    }
}

impl Solution {
    fn sum(min_value: u32, max_value: u32) -> u64 {
        u64::from(min_value + max_value) * u64::from(max_value + 1 - min_value) / 2
    }

    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let mut orders = orders as u32;
        let mut count_counts = HashMap::<u32, u32>::new();

        for count in inventory {
            *count_counts.entry(count as _).or_default() += 1;
        }

        let mut queue = count_counts
            .into_iter()
            .map(|(count, count_count)| Item { count, count_count })
            .collect::<BinaryHeap<_>>();

        let mut result = 0_u64;
        let mut max_item_1 = queue.pop().unwrap();

        while let Some(mut max_item_2) = queue.pop() {
            let available = u64::from(max_item_1.count - max_item_2.count) * u64::from(max_item_1.count_count);

            if available < u64::from(orders) {
                result += Self::sum(max_item_2.count + 1, max_item_1.count) * u64::from(max_item_1.count_count);
                result %= MODULUS;

                orders -= available as u32;
                max_item_2.count_count += max_item_1.count_count;

                max_item_1 = max_item_2;
            } else {
                break;
            }
        }

        let full_cycles = orders / max_item_1.count_count;
        let remainder = orders % max_item_1.count_count;
        let min_value = max_item_1.count - full_cycles;

        result += Self::sum(min_value + 1, max_item_1.count) * u64::from(max_item_1.count_count);
        result += u64::from(min_value) * u64::from(remainder);
        result %= MODULUS;

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        Self::max_profit(inventory, orders)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                count: 2,
                count_count: 3
            } == Item {
                count: 2,
                count_count: 5
            }
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
