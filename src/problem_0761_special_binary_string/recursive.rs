pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;
use std::{iter, mem};

struct Node {
    children: Vec<Self>,
}

enum State<'a> {
    Start(&'a [Node]),
    NextChild(Iter<'a, Node>),
    End,
}

impl Node {
    fn forest_from_bytes(mut input: &[u8]) -> (Vec<Self>, &[u8]) {
        let mut children = Vec::new();

        while let Some((child, next_input)) = Self::from_bytes(input) {
            children.push(child);
            input = next_input;
        }

        (children, input)
    }

    fn from_bytes(input: &[u8]) -> Option<(Self, &[u8])> {
        if let Some((b'1', current_input)) = input.split_first() {
            let (children, current_input) = Self::forest_from_bytes(current_input);

            Some((Self { children }, &input[input.len() - (current_input.len() - 1)..]))
        } else {
            None
        }
    }

    #[allow(clippy::manual_map, clippy::option_if_let_else)]
    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let mut state = State::Start(&self.children);
        let mut stack = Vec::new();

        iter::from_fn(move || match &mut state {
            State::Start(children) => {
                state = State::NextChild(children.iter());

                Some(b'1')
            }
            State::NextChild(child_iter) => Some(if let Some(child) = child_iter.next() {
                stack.push(mem::replace(child_iter, child.children.iter()));

                b'1'
            } else {
                state = State::End;

                b'0'
            }),
            State::End => {
                if let Some(child_iter) = stack.last_mut() {
                    Some(if let Some(child) = child_iter.next() {
                        state = State::NextChild(child.children.iter());

                        b'1'
                    } else {
                        stack.pop();

                        b'0'
                    })
                } else {
                    None
                }
            }
        })
    }
}

impl Solution {
    fn sort_forest(children: &mut [Node]) {
        children
            .iter_mut()
            .for_each(|node| Self::sort_forest(&mut node.children));

        children.sort_unstable_by(|lhs, rhs| rhs.iter().cmp(lhs.iter()));
    }

    pub fn make_largest_special(s: String) -> String {
        let mut s = s.into_bytes();
        let mut forest = Node::forest_from_bytes(&s).0;

        Self::sort_forest(&mut forest);

        for (target, source) in s.iter_mut().zip(forest.iter().flat_map(Node::iter)) {
            *target = source;
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_largest_special(s: String) -> String {
        Self::make_largest_special(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
