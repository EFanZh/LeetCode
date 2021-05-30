// ------------------------------------------------------ snip ------------------------------------------------------ //

struct MyCircularQueue {
    buffer: Box<[i32]>,
    start: usize,
    length: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            buffer: vec![0; k as _].into_boxed_slice(),
            start: 0,
            length: 0,
        }
    }

    fn buffer_index(&self, index: usize) -> usize {
        let buffer_length = self.buffer.len();

        if index < buffer_length - self.start {
            self.start + index
        } else {
            index - (buffer_length - self.start)
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.buffer[self.buffer_index(self.length)] = value;
            self.length += 1;

            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.start += 1;

            if self.start == self.buffer.len() {
                self.start = 0;
            }

            self.length -= 1;

            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buffer[self.start]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buffer[self.buffer_index(self.length - 1)]
        }
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn is_full(&self) -> bool {
        self.length == self.buffer.len()
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
