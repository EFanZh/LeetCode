pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut graph = HashMap::new();

        // Build graph.

        for word in word_list.iter().map(String::as_bytes) {
            for i in 0..word.len() {
                let key = (&word[..i], &word[i + 1..]);

                match graph.entry(key) {
                    Entry::Vacant(entry) => {
                        entry.insert(vec![word]);
                    }
                    Entry::Occupied(entry) => {
                        entry.into_mut().push(word);
                    }
                }
            }
        }

        // BFS.

        let mut queue = iter::once(begin_word.as_bytes()).collect::<VecDeque<_>>();
        let mut visited = iter::once(begin_word.as_bytes()).collect::<HashSet<_>>();
        let mut length = 1;

        loop {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                if current == end_word.as_bytes() {
                    return length;
                } else {
                    for i in 0..current.len() {
                        if let Some(nexts) = graph.get(&(&current[..i], &current[i + 1..])) {
                            for next in nexts {
                                if visited.insert(next) {
                                    queue.push_back(next);
                                }
                            }
                        }
                    }
                }
            }

            if queue.is_empty() {
                break;
            } else {
                length += 1;
            }
        }

        0
    }
}

impl super::Solution for Solution {
    fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        Self::ladder_length(begin_word, end_word, word_list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
