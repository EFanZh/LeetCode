// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

pub struct MyStack {
    q: VecDeque<i32>, // Stores all elements except the top element pushed onto the stack.
    t: VecDeque<i32>, // Optionally stores the top element.
}

impl MyStack {
    fn new() -> Self {
        Self {
            q: VecDeque::new(),
            t: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if let Some(value) = self.t.pop_front() {
            self.q.push_back(value);
        }

        self.t.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        for _ in 1..self.q.len() {
            self.t.push_back(self.q.pop_front().unwrap());
        }

        mem::swap(&mut self.q, &mut self.t);

        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.t.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.t.is_empty()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyStack>();
    }
}
