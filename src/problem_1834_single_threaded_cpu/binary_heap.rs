pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = (0_u32..)
            .zip(tasks)
            .map(|(i, task)| {
                let [enqueue_time, processing_time] = task.try_into().ok().unwrap();

                (enqueue_time as u32, processing_time as u32, i)
            })
            .collect::<Box<_>>();

        tasks.sort_unstable();

        let mut result = Vec::with_capacity(tasks.len());
        let mut queue = BinaryHeap::<Reverse<(u32, u32)>>::new();
        let mut next_idle = 0;

        for &(enqueue_time, processing_time, i) in &*tasks {
            loop {
                if next_idle < enqueue_time {
                    if let Some(Reverse((top_process_time, top_index))) = queue.pop() {
                        next_idle += top_process_time;
                        result.push(top_index as _);
                    } else {
                        next_idle = enqueue_time + processing_time;
                        result.push(i as _);

                        break;
                    }
                } else {
                    queue.push(Reverse((processing_time, i)));

                    break;
                }
            }
        }

        drop(tasks);

        while let Some(Reverse((_, i))) = queue.pop() {
            result.push(i as _);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        Self::get_order(tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
