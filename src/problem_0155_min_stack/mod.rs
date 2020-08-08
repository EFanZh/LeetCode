pub mod one_stack;

pub trait MinStack {
    fn new() -> Self;
    fn push(&mut self, x: i32);
    fn pop(&mut self);
    fn top(&self) -> i32;
    fn get_min(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    enum Operation {
        Push(i32),
        Pop,
        Top(i32),
        GetMin(i32),
    }

    pub fn run<S: MinStack>() {
        use Operation::{GetMin, Pop, Push, Top};

        let test_cases = [(&[Push(-2), Push(0), Push(-3), GetMin(-3), Pop, Top(0), GetMin(-2)] as &[_])];

        for operations in test_cases.iter().copied() {
            let mut stack = S::new();

            for operation in operations {
                match operation {
                    Push(value) => stack.push(*value),
                    Pop => stack.pop(),
                    Top(expected) => assert_eq!(stack.top(), *expected),
                    GetMin(expected) => assert_eq!(stack.get_min(), *expected),
                }
            }
        }
    }
}
