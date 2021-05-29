pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::iter;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if word_list.contains(&end_word) {
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

            let mut left_queue = iter::once((begin_word.as_bytes(), 1)).collect::<VecDeque<_>>();
            let mut right_queue = iter::once((end_word.as_bytes(), 1)).collect::<VecDeque<_>>();
            let mut left_visited = iter::once((begin_word.as_bytes(), 1)).collect::<HashMap<_, _>>();
            let mut right_visited = iter::once((end_word.as_bytes(), 1)).collect::<HashMap<_, _>>();

            while let Some((left, left_length)) = left_queue.pop_front() {
                for i in 0..left.len() {
                    if let Some(nexts) = graph.get(&(&left[..i], &left[i + 1..])) {
                        for next in nexts {
                            if let Some(right_length) = right_visited.get(next) {
                                return left_length + right_length;
                            } else if let Entry::Vacant(entry) = left_visited.entry(next) {
                                entry.insert(left_length + 1);
                                left_queue.push_back((next, left_length + 1));
                            }
                        }
                    }
                }

                if let Some((right, right_length)) = right_queue.pop_front() {
                    for i in 0..left.len() {
                        for next in &graph[&(&right[..i], &right[i + 1..])] {
                            if let Some(left_length) = left_visited.get(next) {
                                return left_length + right_length;
                            } else if let Entry::Vacant(entry) = right_visited.entry(next) {
                                entry.insert(right_length + 1);
                                right_queue.push_back((next, right_length + 1));
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
