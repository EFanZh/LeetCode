pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

struct Queue {
    front: i32,
    rest: VecDeque<i32>,
}

impl Queue {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            front: i32::MIN,
            rest: VecDeque::with_capacity(capacity),
        }
    }

    fn push_back(&mut self, value: i32) {
        let Self { front, rest } = self;

        loop {
            if let Some(&back) = rest.back() {
                if back < value {
                    rest.pop_back();
                } else {
                    break;
                }
            } else {
                if *front < value {
                    *front = value;

                    return;
                }

                break;
            }
        }

        self.rest.push_back(value);
    }

    fn slide(&mut self, old: i32, new: i32) {
        if self.front == old {
            if let Some(new_front) = self.rest.pop_front() {
                self.front = new_front;
            } else {
                self.front = new;

                return;
            }
        }

        self.push_back(new);
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as _;
        let (left, right) = nums.split_at(k);
        let mut queue = Queue::with_capacity(k - 1);
        let mut result = Vec::with_capacity(nums.len() - (k - 1));

        for &num in left {
            queue.push_back(num);
        }

        result.push(queue.front);

        for (&old, &new) in nums.iter().zip(right) {
            queue.slide(old, new);
            result.push(queue.front);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_sliding_window(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
