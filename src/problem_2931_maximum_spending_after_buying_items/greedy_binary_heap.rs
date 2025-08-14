pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;

struct Item {
    key: u32,
    row: Vec<u32>,
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
        other.key.cmp(&self.key)
    }
}

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut queue = values
            .into_iter()
            .map(|row| {
                let mut row = row.into_iter().map(|x| x as u32).collect::<Vec<_>>();
                let key = row.pop().unwrap();

                Item { key, row }
            })
            .collect::<BinaryHeap<_>>();

        let mut result = 0;
        let mut day = 0;

        while let Some(mut peek_mut) = queue.peek_mut() {
            day += 1;
            result += u64::from(peek_mut.key) * day;

            if let Some(next_key) = peek_mut.row.pop() {
                peek_mut.key = next_key;
            } else {
                PeekMut::pop(peek_mut);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        Self::max_spending(values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
