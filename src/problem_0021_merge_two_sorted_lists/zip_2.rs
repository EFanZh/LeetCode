use super::super::data_structures::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        if let Some(mut l1_box) = l1 {
            let mut target = &mut result;

            while let Some(mut l2_box) = l2 {
                if l1_box.val <= l2_box.val {
                    l2 = l1_box.next.take();
                    *target = Some(l1_box);
                    l1_box = l2_box;
                } else {
                    l2 = l2_box.next.take();
                    *target = Some(l2_box);
                }

                target = &mut target.as_mut().unwrap().next;
            }

            *target = Some(l1_box);
        } else {
            result = l2;
        }

        result
    }
}

impl super::Solution for Solution {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_two_lists(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
