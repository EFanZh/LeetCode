// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::iter;

pub struct OrderedStream {
    start: usize,
    queue: VecDeque<Option<String>>,
}

impl OrderedStream {
    fn new(_n: i32) -> Self {
        Self {
            start: 1,
            queue: VecDeque::new(),
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let index = id_key as u32 as usize - self.start;
        let mut result = Vec::new();

        if index == 0 {
            result.push(value);

            self.queue.pop_front();

            loop {
                self.start += 1;

                if self.queue.front().map_or(false, Option::is_some) {
                    result.push(self.queue.pop_front().unwrap().unwrap());
                } else {
                    break;
                }
            }
        } else if let Some(slot) = self.queue.get_mut(index) {
            slot.replace(value);
        } else {
            self.queue
                .extend(iter::repeat_with(|| None).take(index - self.queue.len()));

            self.queue.push_back(Some(value));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::OrderedStream for OrderedStream {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.insert(id_key, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::OrderedStream>();
    }
}
