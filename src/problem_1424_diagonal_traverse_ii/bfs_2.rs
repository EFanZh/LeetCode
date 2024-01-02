pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::iter::Copied;
use std::slice::Iter;

enum Node<'a> {
    First(Iter<'a, Vec<i32>>),
    Rest(Copied<Iter<'a, i32>>),
}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut node = Node::First(nums.iter());

        loop {
            'block: {
                let mut iter = match node {
                    Node::First(mut iter) => {
                        if let Some(row) = iter.next() {
                            queue.push_back(Node::First(iter));

                            row.iter().copied()
                        } else {
                            break 'block;
                        }
                    }
                    Node::Rest(iter) => iter,
                };

                if let Some(num) = iter.next() {
                    queue.push_back(Node::Rest(iter));
                    result.push(num);
                }
            }

            if let Some(new_node) = queue.pop_front() {
                node = new_node;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_diagonal_order(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
