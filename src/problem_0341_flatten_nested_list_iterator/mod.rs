use super::data_structures::NestedInteger;

pub mod stack;
pub mod stack_2;

pub trait NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self;
    fn next(&mut self) -> i32;
    fn has_next(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::super::data_structures::NestedInteger;
    use super::NestedIterator;
    use std::iter;

    pub fn run<I: NestedIterator>() {
        let test_cases = vec![
            (
                vec![
                    NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
                    NestedInteger::Int(2),
                    NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
                ],
                &[1, 1, 2, 1, 1] as &[_],
            ),
            (
                vec![
                    NestedInteger::Int(1),
                    NestedInteger::List(vec![
                        NestedInteger::Int(4),
                        NestedInteger::List(vec![NestedInteger::Int(6)]),
                    ]),
                ],
                &[1, 4, 6],
            ),
        ];

        for (nested_list, expected) in test_cases {
            let mut nested_iterator = I::new(nested_list);

            let result = iter::from_fn(|| {
                if nested_iterator.has_next() {
                    Some(nested_iterator.next())
                } else {
                    None
                }
            })
            .collect::<Box<_>>();

            assert_eq!(result.as_ref(), expected);
        }
    }
}
