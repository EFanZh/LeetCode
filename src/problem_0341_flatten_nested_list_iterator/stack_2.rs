use crate::data_structures::NestedInteger;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::vec::IntoIter;

pub struct NestedIterator {
    state: Option<(i32, IntoIter<NestedInteger>, Vec<IntoIter<NestedInteger>>)>,
}

impl NestedIterator {
    fn next_state(iter: &mut IntoIter<NestedInteger>, stack: &mut Vec<IntoIter<NestedInteger>>) -> Option<i32> {
        loop {
            if let Some(first) = iter.next() {
                match first {
                    NestedInteger::Int(value) => return Some(value),
                    NestedInteger::List(children) => stack.push(mem::replace(iter, children.into_iter())),
                }
            } else if let Some(next_iter) = stack.pop() {
                *iter = next_iter;
            } else {
                break;
            }
        }

        None
    }

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut iter = nested_list.into_iter();
        let mut stack = Vec::new();

        Self {
            state: Self::next_state(&mut iter, &mut stack).map(|value| (value, iter, stack)),
        }
    }

    fn next(&mut self) -> i32 {
        let (result_ref, iter, stack) = self.state.as_mut().unwrap();
        let result = *result_ref;

        if let Some(next_result) = Self::next_state(iter, stack) {
            *result_ref = next_result;
        } else {
            self.state = None;
        }

        result
    }

    fn has_next(&self) -> bool {
        self.state.is_some()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::NestedIterator for NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self::new(nested_list)
    }

    fn next(&mut self) -> i32 {
        self.next()
    }

    fn has_next(&self) -> bool {
        self.has_next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NestedIterator>();
    }
}
