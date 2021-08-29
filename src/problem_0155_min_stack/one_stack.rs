// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let min = self.stack.last().map_or(x, |&(_, min)| min.min(x));

        self.stack.push((x, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MinStack for MinStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) {
        self.pop();
    }

    fn top(&self) -> i32 {
        self.top()
    }

    fn get_min(&self) -> i32 {
        self.get_min()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MinStack>();
    }
}
