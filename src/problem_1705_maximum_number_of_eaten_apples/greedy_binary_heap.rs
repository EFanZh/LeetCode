pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

struct Item {
    deadline: u16,
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
        other.deadline.cmp(&self.deadline)
    }
}

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut queue = BinaryHeap::new();
        let mut result = 0;
        let mut i = 0;

        for (count, day) in apples.into_iter().zip(days) {
            queue.push(Item {
                deadline: i + day as u16,
                count: count as _,
            });

            while let Some(mut top) = queue.peek_mut() {
                if i < top.deadline {
                    result += 1;

                    if top.count == 1 {
                        PeekMut::pop(top);
                    } else {
                        top.count -= 1;
                    }

                    break;
                }

                PeekMut::pop(top);
            }

            i += 1;
        }

        while let Some(top) = queue.pop() {
            let count = top.count.min(top.deadline - i);

            result += i32::from(count);

            i += count;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        Self::eaten_apples(apples, days)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { deadline: 2, count: 3 } == Item { deadline: 2, count: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
