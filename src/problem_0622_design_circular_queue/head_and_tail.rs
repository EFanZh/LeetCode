// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct MyCircularQueue {
    buffer: Box<[i32]>,
    head: usize,
    tail: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            buffer: vec![0; (k + 1) as _].into_boxed_slice(),
            head: 0,
            tail: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.buffer[self.tail] = value;
            self.tail += 1;

            if self.tail == self.buffer.len() {
                self.tail = 0;
            }

            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head += 1;

            if self.head == self.buffer.len() {
                self.head = 0;
            }

            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buffer[self.head]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else if self.tail == 0 {
            *self.buffer.last().unwrap()
        } else {
            self.buffer[self.tail - 1]
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        if self.head == 0 {
            self.tail + 1 == self.buffer.len()
        } else {
            self.tail + 1 == self.head
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyCircularQueue for MyCircularQueue {
    fn new(k: i32) -> Self {
        Self::new(k)
    }

    fn en_queue(&mut self, value: i32) -> bool {
        self.en_queue(value)
    }

    fn de_queue(&mut self) -> bool {
        self.de_queue()
    }

    fn front(&self) -> i32 {
        self.front()
    }

    fn rear(&self) -> i32 {
        self.rear()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn is_full(&self) -> bool {
        self.is_full()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyCircularQueue>();
    }
}
