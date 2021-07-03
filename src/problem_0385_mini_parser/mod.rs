use crate::data_structures::NestedInteger;

pub mod recursive;

pub trait Solution {
    fn deserialize(s: String) -> NestedInteger;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structures::NestedInteger;

    pub fn run<S: Solution>() {
        use NestedInteger::{Int, List};

        let test_cases = [
            ("324", Int(324)),
            (
                "[123,[456,[789]]]",
                List(vec![Int(123), List(vec![Int(456), List(vec![Int(789)])])]),
            ),
            ("[1,23,-45]", List(vec![Int(1), Int(23), Int(-45)])),
            (
                "[123,456,[788,799,833],[[]],10,[]]",
                List(vec![
                    Int(123),
                    Int(456),
                    List(vec![Int(788), Int(799), Int(833)]),
                    List(vec![List(vec![])]),
                    Int(10),
                    List(vec![]),
                ]),
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::deserialize(s.to_string()), expected);
        }
    }
}
