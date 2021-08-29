// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct MyCircularDeque {
    buffer: Box<[i32]>,
    head: usize,
    tail: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            buffer: vec![0; (k + 1) as _].into_boxed_slice(),
            head: 0,
            tail: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if self.head == 0 {
                self.head = self.buffer.len() - 1;
            } else {
                self.head -= 1;
            }

            self.buffer[self.head] = value;

            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
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

    fn delete_front(&mut self) -> bool {
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

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            if self.tail == 0 {
                self.tail = self.buffer.len() - 1;
            } else {
                self.tail -= 1;
            }

            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buffer[self.head]
        }
    }

    fn get_rear(&self) -> i32 {
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
