use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

enum State<'a> {
    Ok(&'a [i32]),
    Error,
    Fatal,
}

impl Solution {
    fn helper<'a>(node: Option<&RefCell<TreeNode>>, voyage: &'a [i32], result: &mut Vec<i32>) -> State<'a> {
        node.map_or(State::Ok(voyage), |node| {
            let node = node.borrow();
            let (&first, voyage) = voyage.split_first().unwrap();

            if node.val == first {
                match Self::helper(node.left.as_deref(), voyage, result) {
                    State::Ok(voyage) => match Self::helper(node.right.as_deref(), voyage, result) {
                        State::Ok(voyage) => State::Ok(voyage),
                        State::Error | State::Fatal => State::Fatal,
                    },
                    State::Error => match Self::helper(node.right.as_deref(), voyage, result) {
                        State::Ok(voyage) => match Self::helper(node.left.as_deref(), voyage, result) {
                            State::Ok(voyage) => {
                                result.push(first);

                                State::Ok(voyage)
                            }
                            State::Error | State::Fatal => State::Fatal,
                        },
                        State::Error | State::Fatal => State::Fatal,
                    },
                    State::Fatal => State::Fatal,
                }
            } else {
                State::Error
            }
        })
    }

    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        match Self::helper(root.as_deref(), &voyage, &mut result) {
            State::Ok(_) => {}
            State::Error | State::Fatal => {
                result.clear();
                result.push(-1);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        Self::flip_match_voyage(root, voyage)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
