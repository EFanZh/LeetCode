pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, mut n: i32) -> i32 {
        n += 1;

        let mut counts = [0; 26];

        for task in tasks {
            counts[usize::from(task as u8 - b'A')] += 1;
        }

        let mut ready_queue = counts
            .iter()
            .copied()
            .filter(|&count| count != 0)
            .collect::<BinaryHeap<_>>();

        let mut pending_queue = VecDeque::<(i32, i32)>::new();
        let mut time = 0;

        loop {
            let count = if let Some(&(task_time, count)) = pending_queue.front() {
                let min_time = task_time + n;

                if time == min_time {
                    pending_queue.pop_front();
                    ready_queue.push(count);
                    ready_queue.pop().unwrap()
                } else {
                    ready_queue.pop().unwrap_or_else(|| {
                        time = min_time;
                        pending_queue.pop_front();

                        count
                    })
                }
            } else if let Some(count) = ready_queue.pop() {
                count
            } else {
                break;
            };

            if count != 1 {
                pending_queue.push_back((time, count - 1));
            }

            time += 1;
        }

        time
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        Self::least_interval(tasks, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
