// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{mem, str};

pub struct SORTracker {
    head: BinaryHeap<(Reverse<u32>, [u8; 10])>,
    tail: BinaryHeap<(u32, Reverse<[u8; 10]>)>,
}

impl SORTracker {
    fn new() -> Self {
        Self {
            head: BinaryHeap::new(),
            tail: BinaryHeap::new(),
        }
    }

    fn add(&mut self, name: String, score: i32) {
        fn parse_name(name: String) -> [u8; 10] {
            let mut result = [0; 10];

            result[..name.len()].copy_from_slice(name.as_bytes());

            result
        }

        let name = parse_name(name);
        let score = score as u32;

        let item = self
            .head
            .peek_mut()
            .filter(|top| (score, Reverse(name)) > (top.0 .0, Reverse(top.1)))
            .map_or((score, Reverse(name)), |mut top| {
                let (old_score, old_name) = mem::replace(&mut *top, (Reverse(score), name));

                (old_score.0, Reverse(old_name))
            });

        self.tail.push(item);
    }

    fn get(&mut self) -> String {
        let (score, Reverse(name)) = self.tail.pop().unwrap();

        self.head.push((Reverse(score), name));

        let length = name.iter().position(|&c| c == 0).unwrap_or(10);

        str::from_utf8(&name[..length]).unwrap().to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SORTracker for SORTracker {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, name: String, score: i32) {
        self.add(name, score);
    }

    fn get(&mut self) -> String {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SORTracker>();
    }
}
