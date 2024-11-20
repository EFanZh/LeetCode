use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let m = m as u32 as usize;
        let n = n as u32 as usize;
        let mut result = vec![vec![-1; n]; m];
        let mut top = 0;
        let mut bottom = m;
        let mut left = 0;
        let mut right = n;
        let mut node = head;

        'outer: loop {
            // Right.

            for target in &mut result[top][left..right] {
                if let Some(boxed_node) = node {
                    node = boxed_node.next;
                    *target = boxed_node.val;
                } else {
                    break 'outer;
                }
            }

            top += 1;

            if top == bottom {
                break;
            }

            // Down.

            for row in &mut result[top..bottom] {
                if let Some(boxed_node) = node {
                    node = boxed_node.next;
                    row[right - 1] = boxed_node.val;
                } else {
                    break 'outer;
                }
            }

            right -= 1;

            if left == right {
                break;
            }

            // Left.

            for target in &mut result[bottom - 1][left..right].iter_mut().rev() {
                if let Some(boxed_node) = node {
                    node = boxed_node.next;
                    *target = boxed_node.val;
                } else {
                    break 'outer;
                }
            }

            bottom -= 1;

            if top == bottom {
                break;
            }

            // Up.

            for row in &mut result[top..bottom].iter_mut().rev() {
                if let Some(boxed_node) = node {
                    node = boxed_node.next;
                    row[left] = boxed_node.val;
                } else {
                    break 'outer;
                }
            }

            left += 1;

            if left == right {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        Self::spiral_matrix(m, n, head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
