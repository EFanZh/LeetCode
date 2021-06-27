pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::Copied;
use std::slice::Iter;

struct Item<'a> {
    value: i32,
    iter: Copied<Iter<'a, i32>>,
}

impl<'a> Item<'a> {
    fn new(slice: &'a [i32]) -> Self {
        let mut iter = slice.iter().copied();
        let value = iter.next().unwrap();

        Item { value, iter }
    }
}

impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl Eq for Item<'_> {}

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.value, &other.value).reverse()
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut queue = nums.iter().map(|list| Item::new(list)).collect::<BinaryHeap<_>>();
        let mut high = queue.iter().map(|item| item.value).max().unwrap();
        let mut best_low = 0;
        let mut best_high = i32::MAX;

        loop {
            let mut top = queue.peek_mut().unwrap();

            if high - top.value < best_high - best_low {
                best_low = top.value;
                best_high = high;
            }

            if let Some(next) = top.iter.next() {
                top.value = next;
                high = high.max(next);
            } else {
                break;
            }
        }

        vec![best_low, best_high]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Self::smallest_range(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_item_partial_eq() {
        assert!(Item::new(&[2, 3]) == Item::new(&[2, 4, 5]));
    }
}
