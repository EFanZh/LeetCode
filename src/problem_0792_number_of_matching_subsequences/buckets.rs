pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut queues = [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];

        for word in &words {
            let mut iter = word.bytes();
            let first = iter.next().unwrap();

            queues[usize::from(first) - usize::from(b'a')].push_back(iter);
        }

        let max_result = words.len() as i32;
        let mut result = 0;

        for c in s.bytes() {
            let queue_index = usize::from(c) - usize::from(b'a');

            for _ in 0..queues[queue_index].len() {
                let mut iter = queues[queue_index].pop_front().unwrap();

                if let Some(first) = iter.next() {
                    queues[usize::from(first) - usize::from(b'a')].push_back(iter);
                } else {
                    result += 1;

                    if result == max_result {
                        return result;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        Self::num_matching_subseq(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
