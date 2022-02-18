// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

pub struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let min = t - 3000;

        while let Some(&front) = self.queue.front() {
            if front < min {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        self.queue.push_back(t);

        self.queue.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RecentCounter for RecentCounter {
    fn new() -> Self {
        Self::new()
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.ping(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RecentCounter>();
    }
}
