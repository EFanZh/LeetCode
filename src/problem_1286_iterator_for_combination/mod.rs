pub mod iterative;

pub trait CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self;
    fn next(&mut self) -> String;
    fn has_next(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::CombinationIterator;

    enum Operation {
        Next(&'static str),
        HasNext(bool),
    }

    pub fn run<C: CombinationIterator>() {
        let test_cases = [
            (("abc", 2), &[Operation::Next("ab"), Operation::HasNext(true)] as &[_]),
            (
                ("ahijp", 2),
                &[
                    Operation::HasNext(true),
                    Operation::Next("ah"),
                    Operation::Next("ai"),
                    Operation::HasNext(true),
                    Operation::Next("aj"),
                    Operation::HasNext(true),
                    Operation::Next("ap"),
                    Operation::HasNext(true),
                    Operation::Next("hi"),
                    Operation::Next("hj"),
                ],
            ),
        ];

        for ((characters, combination_length), operations) in test_cases {
            let mut combination_iterator = C::new(characters.to_string(), combination_length);

            for operation in operations {
                match *operation {
                    Operation::Next(expected) => assert_eq!(combination_iterator.next(), expected),
                    Operation::HasNext(expected) => assert_eq!(combination_iterator.has_next(), expected),
                }
            }
        }
    }
}
