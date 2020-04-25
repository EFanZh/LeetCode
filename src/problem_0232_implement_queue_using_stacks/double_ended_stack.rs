pub struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            front: Vec::new(), // Invariant: if the stack is non-empty, `front` is non-empty.
            back: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.empty() {
            self.front.push(x)
        } else {
            self.back.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        let result = self.front.pop().unwrap();

        if self.front.is_empty() {
            while let Some(item) = self.back.pop() {
                self.front.push(item)
            }
        }

        result
    }

    fn peek(&self) -> i32 {
        *self.front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.front.is_empty()
    }
}

impl super::MyQueue for MyQueue {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn peek(&self) -> i32 {
        self.peek()
    }

    fn empty(&self) -> bool {
        self.empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyQueue>();
    }
}
