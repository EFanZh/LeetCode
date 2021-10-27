use crate::data_structures::{ListNode, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter;
use std::rc::Rc;

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

pub fn invert_tree(root: Option<&RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|root| {
        let root = root.borrow();

        Rc::new(RefCell::new(TreeNode {
            val: root.val,
            left: invert_tree(root.left.as_deref()),
            right: invert_tree(root.right.as_deref()),
        }))
    })
}

pub fn iter_list(list: &Option<Box<ListNode>>) -> impl Iterator<Item = &i32> {
    iter::successors(list.as_deref(), |node| node.next.as_deref()).map(|node| &node.val)
}

pub fn iter_tree(root: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = Option<i32>> {
    let mut non_null_nodes = usize::from(root.is_some());
    let mut queue = VecDeque::from(vec![root]);

    iter::from_fn(move || {
        (non_null_nodes != 0).then(|| {
            queue.pop_front().unwrap().map(|node| {
                let node = node.borrow();

                non_null_nodes -= 1;
                non_null_nodes += usize::from(node.left.is_some()) + usize::from(node.right.is_some());
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());

                node.val
            })
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
