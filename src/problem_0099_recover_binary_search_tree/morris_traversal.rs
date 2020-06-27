use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

enum State {
    Initial,
    GotPrev(Rc<RefCell<TreeNode>>, i32),
    GotFirst(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>, i32),
    GotSecond(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>),
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut state = State::Initial;

        if let Some(mut current) = root.clone() {
            'outer: loop {
                let current_ref = current.borrow_mut();

                if let Some(left) = current_ref.left.as_ref() {
                    let mut node = left.clone();

                    loop {
                        let mut node_ref = node.borrow_mut();

                        if let Some(right) = node_ref.right.clone() {
                            if Rc::ptr_eq(&right, &current) {
                                node_ref.right = None;

                                break;
                            } else {
                                drop(node_ref);
                                node = right;
                            }
                        } else {
                            let left = left.clone();

                            drop(current_ref);

                            node_ref.right = Some(mem::replace(&mut current, left));

                            continue 'outer;
                        }
                    }
                }

                let current_val = current_ref.val;
                let maybe_right = current_ref.right.clone();

                drop(current_ref);

                match state {
                    State::Initial => state = State::GotPrev(current, current_val),
                    State::GotPrev(previous, previous_val) => {
                        state = if current_val < previous_val {
                            State::GotFirst(previous, current, current_val)
                        } else {
                            State::GotPrev(current, current_val)
                        }
                    }
                    State::GotFirst(first_left, first_right, previous_val) => {
                        state = if current_val < previous_val {
                            State::GotSecond(first_left, current)
                        } else {
                            State::GotFirst(first_left, first_right, current_val)
                        }
                    }
                    State::GotSecond(..) => {}
                };

                if let Some(right) = maybe_right {
                    current = right;
                } else {
                    break;
                }
            }
        }

        if let State::GotFirst(first, second, _) | State::GotSecond(first, second) = state {
            mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val)
        }
    }
}

impl super::Solution for Solution {
    fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::recover_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
