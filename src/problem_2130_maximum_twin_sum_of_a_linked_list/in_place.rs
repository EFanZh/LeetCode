use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut list = Vec::new();

        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }

        let (left, right) = list.split_at(list.len() / 2);

        left.iter().zip(right.iter().rev()).map(|(x, y)| x + y).max().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        Self::pair_sum(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
