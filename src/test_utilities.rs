use crate::data_structures::{ListNode, TreeNode};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter;
use std::rc::Rc;

#[expect(unnameable_types, reason = "false positive?")]
pub trait Matrix<T>: Debug {
    fn to_vec(&self) -> Vec<Vec<T>>;

    fn equals_to_slice(&self, slice: &[Vec<T>]) -> bool
    where
        T: PartialEq;
}

impl<T, const M: usize, const N: usize> Matrix<T> for [[T; N]; M]
where
    T: Clone + Debug,
{
    fn to_vec(&self) -> Vec<Vec<T>> {
        Matrix::to_vec(self.as_slice())
    }

    fn equals_to_slice(&self, slice: &[Vec<T>]) -> bool
    where
        T: PartialEq,
    {
        self.as_slice().equals_to_slice(slice)
    }
}

impl<T, const N: usize> Matrix<T> for [[T; N]]
where
    T: Clone + Debug,
{
    fn to_vec(&self) -> Vec<Vec<T>> {
        self.iter().map(|row| row.to_vec()).collect()
    }

    fn equals_to_slice(&self, slice: &[Vec<T>]) -> bool
    where
        T: PartialEq,
    {
        *slice == *self
    }
}

impl<'a, T> PartialEq<&'a dyn Matrix<T>> for Vec<Vec<T>>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a dyn Matrix<T>) -> bool {
        other.equals_to_slice(self)
    }
}

#[expect(clippy::ref_option, reason = "by-design")]
pub fn compare_tree(lhs: &Option<Rc<RefCell<TreeNode>>>, rhs: &Option<Rc<RefCell<TreeNode>>>) -> Ordering {
    fn inner(lhs: Option<&RefCell<TreeNode>>, rhs: Option<&RefCell<TreeNode>>) -> Ordering {
        match (lhs, rhs) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(lhs), Some(rhs)) => {
                let lhs = lhs.borrow();
                let rhs = rhs.borrow();

                i32::cmp(&lhs.val, &rhs.val)
                    .then_with(|| inner(lhs.left.as_deref(), rhs.left.as_deref()))
                    .then_with(|| inner(lhs.right.as_deref(), rhs.right.as_deref()))
            }
        }
    }

    inner(lhs.as_deref(), rhs.as_deref())
}

#[expect(clippy::ref_option, reason = "by-design")]
pub fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref().and_then(|root| {
        let root_ref = root.borrow();

        if root_ref.val == val {
            Some(Rc::clone(root))
        } else {
            find_node(&root_ref.left, val).or_else(|| find_node(&root_ref.right, val))
        }
    })
}

pub fn invert_tree(root: &RefCell<TreeNode>) -> Rc<RefCell<TreeNode>> {
    let root = root.borrow();

    Rc::new(RefCell::new(TreeNode {
        val: root.val,
        left: root.left.as_deref().map(invert_tree),
        right: root.right.as_deref().map(invert_tree),
    }))
}

pub fn is_full_stack_operations(operations: impl IntoIterator<Item = bool>) -> bool {
    let mut depth = 0_usize;

    for operation in operations {
        if operation {
            if depth == 0 {
                return false;
            }

            depth -= 1;
        } else {
            depth += 1;
        }
    }

    depth == 0
}

pub fn is_subsequence<T, U>(lhs: impl IntoIterator<Item = T>, rhs: impl IntoIterator<Item = U>) -> bool
where
    T: PartialEq<U>,
{
    let mut iter = rhs.into_iter();

    for left in lhs {
        if iter.all(|right| left != right) {
            return false;
        }
    }

    true
}

pub fn iter_list(mut list: Option<Box<ListNode>>) -> impl Iterator<Item = i32> {
    iter::from_fn(move || {
        list.take().map(|mut node| {
            list = node.next.take();

            node.val
        })
    })
}

pub fn iter_tree_pre_order(root: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = i32> {
    let mut stack = root.into_iter().collect::<Vec<_>>();

    iter::from_fn(move || {
        stack.pop().map(|node| {
            let node_ref = node.borrow();

            if let Some(right) = node_ref.right.clone() {
                stack.push(right);
            }

            if let Some(left) = node_ref.left.clone() {
                stack.push(left);
            }

            node_ref.val
        })
    })
}

pub fn iter_tree_in_order(root: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = i32> {
    let mut state = root;
    let mut stack = Vec::new();

    iter::from_fn(move || {
        while let Some(node) = state.take() {
            state.clone_from(&node.borrow().left);
            stack.push(node);
        }

        stack.pop().map(|node| {
            let node_ref = node.borrow();

            state.clone_from(&node_ref.right);

            node_ref.val
        })
    })
}

pub fn iter_tree_post_order(root: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = i32> {
    enum State {
        Start(Rc<RefCell<TreeNode>>),
        Left(i32, Rc<RefCell<TreeNode>>),
        Right(i32),
    }

    let mut stack = root.map(State::Start).into_iter().collect::<Vec<_>>();

    iter::from_fn(move || {
        stack.pop().map(|state| {
            let mut node = match state {
                State::Start(root) => root,
                State::Left(value, right) => {
                    stack.push(State::Right(value));

                    right
                }
                State::Right(value) => return value,
            };

            loop {
                let node_ref = node.borrow();

                if let Some(left) = node_ref.left.clone() {
                    stack.push(if let Some(right) = node_ref.right.clone() {
                        State::Left(node_ref.val, right)
                    } else {
                        State::Right(node_ref.val)
                    });

                    drop(node_ref);

                    node = left;
                } else if let Some(right) = node_ref.right.clone() {
                    stack.push(State::Right(node_ref.val));

                    drop(node_ref);

                    node = right;
                } else {
                    return node_ref.val;
                }
            }
        })
    })
}

pub fn make_list<I: IntoIterator<Item = i32>>(values: I) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut node_ref = &mut result;

    for value in values {
        *node_ref = Some(Box::new(ListNode::new(value)));

        node_ref = &mut node_ref.as_mut().unwrap().next;
    }

    result
}

pub fn make_tree<I: IntoIterator<Item = Option<i32>>>(values: I) -> Option<Rc<RefCell<TreeNode>>> {
    let mut iter = values.into_iter();

    iter.next().flatten().map(|first_value| {
        let root = Rc::new(RefCell::new(TreeNode::new(first_value)));
        let mut target = Rc::clone(&root);
        let mut queue = VecDeque::new();

        while let Some(item) = iter.next() {
            let mut target_ref = target.borrow_mut();

            if let Some(value) = item {
                let left = Rc::new(RefCell::new(TreeNode::new(value)));

                target_ref.left = Some(Rc::clone(&left));

                queue.push_back(left);
            }

            if let Some(item) = iter.next() {
                if let Some(value) = item {
                    let right = Rc::new(RefCell::new(TreeNode::new(value)));

                    target_ref.right = Some(Rc::clone(&right));

                    queue.push_back(right);
                }
            } else {
                break;
            }

            drop(target_ref);

            if let Some(next_target) = queue.pop_front() {
                target = next_target;
            } else {
                break;
            }
        }

        root
    })
}

pub fn unstable_sorted_by<T>(iter: impl IntoIterator<Item = T>, compare: impl FnMut(&T, &T) -> Ordering) -> Vec<T> {
    let mut result = iter.into_iter().collect::<Vec<_>>();

    result.sort_unstable_by(compare);

    result
}

pub fn unstable_sorted_by_key<T, K: Ord>(iter: impl IntoIterator<Item = T>, f: impl FnMut(&T) -> K) -> Vec<T> {
    let mut result = iter.into_iter().collect::<Vec<_>>();

    result.sort_unstable_by_key(f);

    result
}

pub fn unstable_sorted<T: Ord>(iter: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut result = iter.into_iter().collect::<Vec<_>>();

    result.sort_unstable();

    result
}

mod tests {
    use std::cmp::Ordering;

    #[test]
    fn test_compare_tree() {
        let test_cases = [
            ((&[Some(1)] as &[_], &[Some(1)] as &[_]), Ordering::Equal),
            ((&[], &[]), Ordering::Equal),
            ((&[Some(1), Some(2)], &[Some(1), Some(2)]), Ordering::Equal),
            ((&[Some(1), None, Some(2)], &[Some(1), None, Some(2)]), Ordering::Equal),
            (
                (&[Some(1), Some(2), Some(3)], &[Some(1), Some(2), Some(3)]),
                Ordering::Equal,
            ),
            ((&[], &[Some(1)]), Ordering::Less),
            ((&[Some(1)], &[Some(3)]), Ordering::Less),
            ((&[Some(1)], &[]), Ordering::Greater),
            ((&[Some(3)], &[Some(1)]), Ordering::Greater),
        ];

        for ((lhs, rhs), expected) in test_cases {
            let lhs = super::make_tree(lhs.iter().copied());
            let rhs = super::make_tree(rhs.iter().copied());

            assert_eq!(super::compare_tree(&lhs, &rhs), expected);
        }
    }

    #[test]
    fn test_is_full_stack_operations() {
        let test_cases = [
            (")", false),
            ("(", false),
            ("", true),
            ("()", true),
            (")(", false),
            ("((()()))", true),
        ];

        for (operations, expected) in test_cases {
            assert_eq!(
                super::is_full_stack_operations(operations.bytes().map(|c| c == b')')),
                expected,
            );
        }
    }

    #[test]
    fn test_is_subsequence() {
        let test_cases = [(("ab", "acb"), true), (("ac", "cab"), false)];

        for ((lhs, rhs), expected) in test_cases {
            assert_eq!(super::is_subsequence(lhs.bytes(), rhs.bytes()), expected);
        }
    }

    #[test]
    fn test_iter_tree_post_order() {
        let test_cases = [
            (
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    None,
                    None,
                    Some(8),
                    None,
                    None,
                    Some(9),
                    Some(10),
                    Some(11),
                ] as &[_],
                &[4, 8, 5, 2, 9, 6, 10, 11, 7, 3, 1] as &[_],
            ),
            (&[], &[]),
            (&[Some(1)], &[1]),
            (&[Some(1), Some(2)], &[2, 1]),
            (&[Some(1), None, Some(2)], &[2, 1]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                super::iter_tree_post_order(super::make_tree(root.iter().copied())).collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
