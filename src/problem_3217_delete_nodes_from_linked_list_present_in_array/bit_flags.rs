use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn insert(buckets: &mut Vec<usize>, value: u32) {
        let index = (value / usize::BITS) as usize;
        let bit = 1 << (value % usize::BITS);

        if let Some(bucket) = buckets.get_mut(index) {
            *bucket |= bit;
        } else {
            let empty_buckets = index - buckets.len();

            buckets.extend(iter::repeat_n(0, empty_buckets).chain(iter::once(bit)));
        }
    }

    fn contains(buckets: &[usize], value: u32) -> bool {
        let index = value / usize::BITS;
        let bit = 1 << (value % usize::BITS);

        buckets.get(index as usize).is_some_and(|&bucket| bucket & bit != 0)
    }

    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut buckets = Vec::<usize>::new();

        for num in nums {
            Self::insert(&mut buckets, num.cast_unsigned());
        }

        let mut head = head;
        let mut cursor = &mut head;

        while let Some(node) = cursor {
            if Self::contains(&buckets, node.val.cast_unsigned()) {
                *cursor = node.next.take();
            } else {
                cursor = &mut cursor.as_deref_mut().unwrap().next; // TODO: Avoid `unwrap()`.
            }
        }

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::modified_list(nums, head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
