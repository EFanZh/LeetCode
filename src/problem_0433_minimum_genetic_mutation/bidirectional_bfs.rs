pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::iter;

impl Solution {
    fn gene_to_u16(gene: &str) -> u16 {
        let mut result = 0;

        for (i, byte) in (0..15).step_by(2).rev().zip(gene.bytes()) {
            match byte {
                b'A' => {}
                b'C' => result |= 1 << i,
                b'G' => result |= 2 << i,
                _ => result |= 3 << i,
            }
        }

        result
    }

    fn iterator_groups(gene: u16) -> impl Iterator<Item = (u16, u16)> {
        (0..15).step_by(2).map(move |i| (i, gene & !(3 << i)))
    }

    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if bank.contains(&end) {
            let start = Self::gene_to_u16(&start);
            let end = Self::gene_to_u16(&end);
            let mut graph = HashMap::new();

            // Build graph.

            for gene in bank.iter().map(String::as_str).map(Self::gene_to_u16) {
                for key in Self::iterator_groups(gene) {
                    match graph.entry(key) {
                        Entry::Vacant(entry) => {
                            entry.insert(vec![gene]);
                        }
                        Entry::Occupied(entry) => {
                            entry.into_mut().push(gene);
                        }
                    }
                }
            }

            // BFS.

            let mut left_queue = iter::once((start, 0)).collect::<VecDeque<_>>();
            let mut right_queue = iter::once((end, 1)).collect::<VecDeque<_>>();
            let mut left_visited = iter::once((start, 0)).collect::<HashMap<_, _>>();
            let mut right_visited = iter::once((end, 1)).collect::<HashMap<_, _>>();

            while let Some((left, left_length)) = left_queue.pop_front() {
                for key in Self::iterator_groups(left) {
                    if let Some(nexts) = graph.get(&key) {
                        for &next in nexts {
                            if let Some(right_length) = right_visited.get(&next) {
                                return left_length + right_length;
                            } else if let Entry::Vacant(entry) = left_visited.entry(next) {
                                entry.insert(left_length + 1);
                                left_queue.push_back((next, left_length + 1));
                            }
                        }
                    }
                }

                if let Some((right, right_length)) = right_queue.pop_front() {
                    for key in Self::iterator_groups(right) {
                        for &next in &graph[&key] {
                            if let Some(left_length) = left_visited.get(&next) {
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

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        Self::min_mutation(start, end, bank)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
