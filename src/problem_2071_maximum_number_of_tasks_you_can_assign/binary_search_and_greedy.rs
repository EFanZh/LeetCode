pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::VecDeque;

impl Solution {
    fn vec_i32_to_vec_u32(v: Vec<i32>) -> Vec<u32> {
        v.into_iter().map(|x| x as _).collect()
    }

    fn check(tasks: &[u32], workers: &[u32], mut pills: u32, strength: u32, buffer: &mut VecDeque<u32>) -> bool {
        let mut tasks_iter = tasks.iter();

        let result = workers.iter().all(|&worker| {
            let enhanced = worker + strength;

            while let Some(&task) = tasks_iter.as_slice().first() {
                if task <= enhanced {
                    tasks_iter.next();
                    buffer.push_back(task);
                } else {
                    break;
                }
            }

            if let Some(&front) = buffer.front() {
                if worker >= front {
                    buffer.pop_front();
                } else {
                    if pills == 0 {
                        return false;
                    }

                    pills -= 1;
                    buffer.pop_back();
                }

                true
            } else {
                false
            }
        });

        buffer.clear();

        result
    }

    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        let mut tasks = Self::vec_i32_to_vec_u32(tasks);
        let mut workers = Self::vec_i32_to_vec_u32(workers);
        let pills = pills as u32;
        let strength = strength as u32;

        let (tasks, workers) = match tasks.len().cmp(&workers.len()) {
            Ordering::Less => {
                let length = tasks.len();

                (
                    tasks.as_mut_slice(),
                    workers.select_nth_unstable_by_key(length, |&x| Reverse(x)).0,
                )
            }
            Ordering::Equal => (tasks.as_mut_slice(), workers.as_mut_slice()),
            Ordering::Greater => {
                let length = workers.len();

                (tasks.select_nth_unstable(length).0, workers.as_mut_slice())
            }
        };

        tasks.sort_unstable();
        workers.sort_unstable();

        let n = tasks.len();
        let mut left = 1;
        let mut right = tasks.len() + 1;
        let mut buffer = VecDeque::new();

        while left < right {
            let middle = usize::midpoint(left, right);

            if Self::check(&tasks[..middle], &workers[n - middle..], pills, strength, &mut buffer) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32 - 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        Self::max_task_assign(tasks, workers, pills, strength)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
