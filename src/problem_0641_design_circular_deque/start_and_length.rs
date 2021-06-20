// ------------------------------------------------------ snip ------------------------------------------------------ //

struct MyCircularDeque {
    buffer: Box<[i32]>,
    start: usize,
    length: usize,
}

impl MyCircularDeque {
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

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if self.start == 0 {
                self.start = self.buffer.len() - 1;
            } else {
                self.start -= 1;
            }

            self.buffer[self.start] = value;
            self.length += 1;

            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.buffer[self.buffer_index(self.length)] = value;
            self.length += 1;

            true
        }
    }

    fn delete_front(&mut self) -> bool {
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

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.length -= 1;

            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buffer[self.start]
        }
    }

    fn get_rear(&self) -> i32 {
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

impl super::MyCircularDeque for MyCircularDeque {
    fn new(k: i32) -> Self {
        Self::new(k)
    }

    fn insert_front(&mut self, value: i32) -> bool {
        self.insert_front(value)
    }

    fn insert_last(&mut self, value: i32) -> bool {
        self.insert_last(value)
    }

    fn delete_front(&mut self) -> bool {
        self.delete_front()
    }

    fn delete_last(&mut self) -> bool {
        self.delete_last()
    }

    fn get_front(&self) -> i32 {
        self.get_front()
    }

    fn get_rear(&self) -> i32 {
        self.get_rear()
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
        super::super::tests::run::<super::MyCircularDeque>();
    }
}
