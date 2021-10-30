use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut l2 = l2;

        if let Some(mut l1_box) = l1 {
            let mut target = &mut result;

            while let Some(mut l2_box) = l2 {
                if l1_box.val <= l2_box.val {
                    l2 = l1_box.next.take();
                    target = &mut target.get_or_insert(l1_box).next; // TODO: Wait for `Option::insert` method.
                    l1_box = l2_box;
                } else {
                    l2 = l2_box.next.take();
                    target = &mut target.get_or_insert(l2_box).next; // TODO: Wait for `Option::insert` method.
                }
            }

            *target = Some(l1_box);
        } else {
            result = l2;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_two_lists(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
