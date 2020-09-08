pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter;
use std::mem;

type ConnectionKey<'a> = (&'a [u8], &'a [u8]);

impl Solution {
    fn build_connection_table(word_list: &[String]) -> HashMap<ConnectionKey, Vec<&[u8]>> {
        let mut connections = HashMap::new();

        for word in word_list.iter().map(String::as_bytes) {
            for i in 0..word.len() {
                let key = (&word[..i], &word[i + 1..]);

                match connections.entry(key) {
                    Entry::Vacant(entry) => {
                        entry.insert(vec![word]);
                    }
                    Entry::Occupied(entry) => {
                        entry.into_mut().push(word);
                    }
                }
            }
        }

        connections
    }

    fn build_connection_graph<'a>(
        begin_word: &'a [u8],
        end_word: &'a [u8],
        connections: &HashMap<ConnectionKey, Vec<&'a [u8]>>,
    ) -> HashMap<&'a [u8], Vec<&'a [u8]>> {
        let mut graph = HashMap::new();
        let mut forward_level = iter::once(begin_word).collect::<HashSet<_>>();
        let mut forward_visited = iter::once(begin_word).collect::<HashSet<_>>();
        let mut backward_level = iter::once(end_word).collect::<HashSet<_>>();
        let mut backward_visited = iter::once(end_word).collect::<HashSet<_>>();
        let mut temp_level = HashSet::new();

        loop {
            let mut connected = false;

            // Forward pass.

            for current in forward_level.drain() {
                let next_slots = graph.entry(current).or_insert_with(Vec::new);

                for i in 0..current.len() {
                    if let Some(nexts) = connections.get(&(&current[..i], &current[i + 1..])) {
                        for &next in nexts {
                            if backward_visited.contains(next) {
                                connected = true;
                            }

                            if !forward_visited.contains(next) {
                                next_slots.push(next);
                                temp_level.insert(next);
                            }
                        }
                    }
                }
            }

            if temp_level.is_empty() || connected {
                break;
            }

            forward_visited.extend(temp_level.iter().copied());

            mem::swap(&mut forward_level, &mut temp_level);

            // Backward pass.

            for current in backward_level.drain() {
                for i in 0..current.len() {
                    if let Some(previouses) = connections.get(&(&current[..i], &current[i + 1..])) {
                        for &previous in previouses {
                            if forward_visited.contains(previous) {
                                connected = true;
                            }

                            if !backward_visited.contains(previous) {
                                graph.entry(previous).or_insert_with(Vec::new).push(current);
                                temp_level.insert(previous);
                            }
                        }
                    }
                }
            }

            if temp_level.is_empty() || connected {
                break;
            }

            backward_visited.extend(temp_level.iter().copied());

            mem::swap(&mut backward_level, &mut temp_level);
        }

        graph
    }

    fn get_paths<'a>(
        begin_word: &'a [u8],
        end_word: &[u8],
        graph: &HashMap<&[u8], Vec<&'a [u8]>>,
        base: &mut Vec<&'a [u8]>,
        result: &mut Vec<Vec<String>>,
    ) {
        base.push(begin_word);

        if begin_word == end_word {
            result.push(
                base.iter()
                    .map(|component| String::from_utf8(component.to_vec()).unwrap())
                    .collect(),
            );
        } else if let Some(nexts) = graph.get(begin_word) {
            for next in nexts {
                Self::get_paths(next, end_word, graph, base, result);
            }
        }

        base.pop();
    }

    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut result = Vec::new();

        if word_list.contains(&end_word) {
            let begin_word = begin_word.as_bytes();
            let end_word = end_word.as_bytes();
            let connections = Self::build_connection_table(&word_list);
            let graph = Self::build_connection_graph(begin_word, end_word, &connections);
            let mut base = Vec::new();

            Self::get_paths(begin_word, end_word, &graph, &mut base, &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        Self::find_ladders(begin_word, end_word, word_list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
