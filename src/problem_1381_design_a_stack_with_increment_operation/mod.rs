pub mod lazy;

pub trait CustomStack {
    fn new(max_size: i32) -> Self;
    fn push(&mut self, x: i32);
    fn pop(&mut self) -> i32;
    fn increment(&mut self, k: i32, val: i32);
}

#[cfg(test)]
mod tests {
    use super::CustomStack;

    enum Operation {
        Push(i32),
        Pop(i32),
        Increment(i32, i32),
    }

    pub fn run<S: CustomStack>() {
        let test_cases = [
            (
                3,
                &[
                    Operation::Push(1),
                    Operation::Push(2),
                    Operation::Pop(2),
                    Operation::Push(2),
                    Operation::Push(3),
                    Operation::Push(4),
                    Operation::Increment(5, 100),
                    Operation::Increment(2, 100),
                    Operation::Pop(103),
                    Operation::Pop(202),
                    Operation::Pop(201),
                    Operation::Pop(-1),
                ] as &[_],
            ),
            (
                2,
                &[
                    Operation::Push(34),
                    Operation::Pop(34),
                    Operation::Increment(8, 100),
                    Operation::Pop(-1),
                    Operation::Increment(9, 91),
                    Operation::Push(63),
                    Operation::Pop(63),
                    Operation::Push(84),
                    Operation::Increment(10, 93),
                    Operation::Increment(6, 45),
                    Operation::Increment(10, 4),
                ],
            ),
        ];

        for (max_size, operations) in test_cases {
            let mut stack = S::new(max_size);

            for operation in operations {
                match *operation {
                    Operation::Pop(expected) => assert_eq!(stack.pop(), expected),
                    Operation::Push(val) => stack.push(val),
                    Operation::Increment(k, val) => stack.increment(k, val),
                }
            }
        }
    }
}
