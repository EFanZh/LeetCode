use super::data_structures::NestedInteger;

pub mod recursive;

pub trait Solution {
    fn deserialize(s: String) -> NestedInteger;
}

#[cfg(test)]
mod tests {
    use super::super::data_structures::NestedInteger;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("324", NestedInteger::Int(324)),
            (
                "[123,[456,[789]]]",
                NestedInteger::List(vec![
                    NestedInteger::Int(123),
                    NestedInteger::List(vec![
                        NestedInteger::Int(456),
                        NestedInteger::List(vec![NestedInteger::Int(789)]),
                    ]),
                ]),
            ),
            (
                "[1,23,-45]",
                NestedInteger::List(vec![
                    NestedInteger::Int(1),
                    NestedInteger::Int(23),
                    NestedInteger::Int(-45),
                ]),
            ),
        ];

        for (s, expected) in test_cases.iter() {
            assert_eq!(S::deserialize(s.to_string()), *expected);
        }
    }
}
