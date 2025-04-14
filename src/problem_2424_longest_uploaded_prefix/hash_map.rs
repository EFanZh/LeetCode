// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

pub struct LUPrefix {
    end_to_start: HashMap<i32, i32>,
    start_to_end: HashMap<i32, i32>,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        let capacity = n as u32 as usize / 2;

        Self {
            end_to_start: HashMap::with_capacity(capacity),
            start_to_end: HashMap::with_capacity(capacity),
        }
    }

    fn upload(&mut self, video: i32) {
        let (start, end) = match (
            self.end_to_start.remove(&(video - 1)),
            self.start_to_end.remove(&(video + 1)),
        ) {
            (None, None) => (video, video),
            (Some(start), None) => (start, video),
            (None, Some(end)) => (video, end),
            (Some(start), Some(end)) => (start, end),
        };

        self.start_to_end.insert(start, end);
        self.end_to_start.insert(end, start);
    }

    fn longest(&self) -> i32 {
        self.start_to_end.get(&1).copied().unwrap_or(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::LUPrefix for LUPrefix {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn upload(&mut self, video: i32) {
        self.upload(video);
    }

    fn longest(&self) -> i32 {
        self.longest()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::LUPrefix>();
    }
}
