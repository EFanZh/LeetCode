pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;

struct Item {
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
        u32::cmp(other.row.last().unwrap(), self.row.last().unwrap())
    }
}

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut queue = values
            .into_iter()
            .map(|row| Item {
                row: row.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>(),
            })
            .collect::<BinaryHeap<_>>();

        let mut result = 0;
        let mut day = 0;

        while let Some(mut peek_mut) = queue.peek_mut() {
            let row = &mut peek_mut.row;

            day += 1;
            result += u64::from(*row.last().unwrap()) * day;
            row.pop();

            if row.is_empty() {
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
