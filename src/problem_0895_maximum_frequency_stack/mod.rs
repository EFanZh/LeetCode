pub mod stack_of_stacks;

pub trait FreqStack {
    fn new() -> Self;
    fn push(&mut self, val: i32);
    fn pop(&mut self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::FreqStack;

    enum Operation {
        Push(i32),
        Pop(i32),
    }

    pub fn run<S: FreqStack>() {
        let test_cases = [(&[
            Operation::Push(5),
            Operation::Push(7),
            Operation::Push(5),
            Operation::Push(7),
            Operation::Push(4),
            Operation::Push(5),
            Operation::Pop(5),
            Operation::Pop(7),
            Operation::Pop(5),
            Operation::Pop(4),
        ] as &[_])];

        for operations in test_cases {
            let mut stack = S::new();

            for operation in operations {
                match *operation {
                    Operation::Pop(expected) => assert_eq!(stack.pop(), expected),
                    Operation::Push(val) => stack.push(val),
                }
            }
        }
    }
}
