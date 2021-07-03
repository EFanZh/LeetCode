use crate::data_structures::NestedInteger;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::vec::IntoIter;

struct NestedIterator {
    state: Option<(i32, IntoIter<NestedInteger>, Vec<IntoIter<NestedInteger>>)>,
}

impl NestedIterator {
    fn next_state(
        mut iter: IntoIter<NestedInteger>,
        mut stack: Vec<IntoIter<NestedInteger>>,
    ) -> Option<(i32, IntoIter<NestedInteger>, Vec<IntoIter<NestedInteger>>)> {
        loop {
            if let Some(first) = iter.next() {
                match first {
                    NestedInteger::Int(value) => return Some((value, iter, stack)),
                    NestedInteger::List(children) => stack.push(mem::replace(&mut iter, children.into_iter())),
                }
            } else if let Some(next_iter) = stack.pop() {
                iter = next_iter;
            } else {
                break;
            }
        }

        None
    }

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            state: Self::next_state(nested_list.into_iter(), Vec::new()),
        }
    }

    fn next(&mut self) -> i32 {
        let (result, iter, stack) = self.state.take().unwrap();

        self.state = Self::next_state(iter, stack);

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
