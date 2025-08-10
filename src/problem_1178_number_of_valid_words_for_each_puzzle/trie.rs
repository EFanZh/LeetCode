pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

struct Node {
    count: usize,
    contains: usize,
    not_contains: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            count: 0,
            contains: usize::MAX,
            not_contains: usize::MAX,
        }
    }
}

impl Solution {
    fn bfs<'a>(pool: &'a [Node], queue: &mut VecDeque<&'a Node>, first: u8, mut state: u32) -> usize {
        queue.push_back(&pool[0]);

        for _ in 0..first {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                if state & 1 != 0
                    && let Some(contains) = pool.get(node.contains)
                {
                    queue.push_back(contains);
                }

                if let Some(not_contains) = pool.get(node.not_contains) {
                    queue.push_back(not_contains);
                }
            }

            state >>= 1;
        }

        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();

            if let Some(contains) = pool.get(node.contains) {
                queue.push_back(contains);
            }
        }

        state >>= 1;

        for _ in first + 1..26 {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                if state & 1 != 0
                    && let Some(contains) = pool.get(node.contains)
                {
                    queue.push_back(contains);
                }

                if let Some(not_contains) = pool.get(node.not_contains) {
                    queue.push_back(not_contains);
                }
            }

            state >>= 1;
        }

        queue.drain(..).map(|node| node.count).sum()
    }

    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut pool = vec![Node::default()];

        for word in words {
            let mut state = 0_u32;

            for c in word.into_bytes() {
                state |= 1 << (c - b'a');
            }

            let mut node_index = 0;

            for i in 0..26 {
                let pool_length = pool.len();
                let node = &mut pool[node_index];

                let child_index = if state & (1 << i) == 0 {
                    &mut node.not_contains
                } else {
                    &mut node.contains
                };

                let child_index = if *child_index < pool_length {
                    *child_index
                } else {
                    *child_index = pool_length;

                    pool.push(Node::default());

                    pool_length
                };

                node_index = child_index;
            }

            pool[node_index].count += 1;
        }

        let mut queue = VecDeque::new();

        puzzles
            .into_iter()
            .map(|puzzle| {
                let mut iter = puzzle.into_bytes().into_iter().map(|c| c - b'a');
                let first = iter.next().unwrap();
                let mut state = 1 << first;

                for c in iter {
                    state |= 1 << c;
                }

                Self::bfs(&pool, &mut queue, first, state) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        Self::find_num_of_valid_words(words, puzzles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
