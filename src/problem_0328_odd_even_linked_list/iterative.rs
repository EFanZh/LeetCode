use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd = None;
        let mut odd_tail = &mut odd;
        let mut even = None;
        let mut even_tail = &mut even;
        let mut head = head;

        while let Some(mut first) = head {
            if let Some(mut second) = first.next.take() {
                head = second.next.take();
                odd_tail = &mut odd_tail.get_or_insert(first).next;
                even_tail = &mut even_tail.get_or_insert(second).next;
            } else {
                odd_tail = &mut odd_tail.get_or_insert(first).next;

                break;
            }
        }

        *odd_tail = even;

        odd
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::odd_even_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
