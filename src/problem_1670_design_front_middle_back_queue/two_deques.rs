// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

pub struct FrontMiddleBackQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            front: VecDeque::new(),
            back: VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }

        self.front.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }

        self.front.push_back(val);
    }

    fn push_back(&mut self, val: i32) {
        if self.front.len() < self.back.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }

        self.back.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        if self.front.len() < self.back.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }

        self.front.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        if self.front.len() < self.back.len() {
            self.back.pop_front()
        } else {
            self.front.pop_back()
        }
        .unwrap_or(-1)
    }

    fn pop_back(&mut self) -> i32 {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }

        self.back.pop_back().unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FrontMiddleBackQueue for FrontMiddleBackQueue {
    fn new() -> Self {
        Self::new()
    }

    fn push_front(&mut self, val: i32) {
        self.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        self.push_middle(val);
    }

    fn push_back(&mut self, val: i32) {
        self.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        self.pop_front()
    }

    fn pop_middle(&mut self) -> i32 {
        self.pop_middle()
    }

    fn pop_back(&mut self) -> i32 {
        self.pop_back()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FrontMiddleBackQueue>();
    }
}
