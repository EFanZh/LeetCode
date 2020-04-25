use std::collections::VecDeque;

pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.q.push_back(x);

        for _ in 1..self.q.len() {
            let value = self.q.pop_front().unwrap();

            self.q.push_back(value)
        }
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

impl super::MyStack for MyStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn top(&self) -> i32 {
        self.top()
    }

    fn empty(&self) -> bool {
        self.empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::MyStack;

    #[test]
    fn test_solution() {
        run::<MyStack>();
    }
}
