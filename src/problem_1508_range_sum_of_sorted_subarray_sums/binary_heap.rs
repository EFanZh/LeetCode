pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;

struct Item {
    sum: u32,
    end: u16,
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
        other.sum.cmp(&self.sum)
    }
}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, _n: i32, left: i32, right: i32) -> i32 {
        let left = left as u32;
        let right = right as u32;

        let mut queue = (1..)
            .zip(&nums)
            .map(|(end, &num)| Item { sum: num as _, end })
            .collect::<BinaryHeap<_>>();

        let mut next_sum = move || {
            let mut item = queue.peek_mut().unwrap();
            let item_ref = &*item;
            let result = item_ref.sum;

            if let Some(&num) = nums.get(usize::from(item_ref.end)) {
                let item_ref_mut = &mut *item;

                item_ref_mut.sum += num as u32;
                item_ref_mut.end += 1;

                drop(item);
            } else {
                PeekMut::pop(item);
            }

            result
        };

        for _ in 1..left {
            next_sum();
        }

        let mut sum = 0;

        for _ in left..=right {
            sum += u64::from(next_sum());
        }

        (sum % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        Self::range_sum(nums, n, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { sum: 2, end: 3 } == Item { sum: 2, end: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
