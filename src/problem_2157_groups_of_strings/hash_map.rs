pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        // Build graph.

        let mut nodes = Vec::<(Cell<u16>, Vec<u16>)>::new();
        let mut word_ids = HashMap::<u32, u16>::new();
        let mut buckets = HashMap::<u32, u16>::new();

        for word in words {
            let word = word
                .into_bytes()
                .into_iter()
                .fold(0_u32, |word, c| word | 1 << (c - b'a'));

            match word_ids.entry(word) {
                Entry::Occupied(entry) => *nodes[usize::from(*entry.get())].0.get_mut() += 1,
                Entry::Vacant(entry) => {
                    let word_id = nodes.len() as _;
                    let mut node_neighbors = Vec::with_capacity(word.count_ones() as usize);
                    let mut word_bits = word;

                    while word_bits != 0 {
                        let bit = word_bits & word_bits.wrapping_neg();
                        let group = word ^ bit;

                        match buckets.entry(group) {
                            Entry::Occupied(entry) => {
                                // Link numbers connected by replacing a letter.

                                let sibling = *entry.get();

                                node_neighbors.push(sibling);
                                nodes[usize::from(sibling)].1.push(word_id);
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(word_id);
                            }
                        }

                        word_bits ^= bit;
                    }

                    entry.insert(word_id);
                    nodes.push((Cell::new(1), node_neighbors));
                }
            };
        }

        for (key, lhs) in buckets {
            if let Some(&rhs) = word_ids.get(&key) {
                // Link numbers connected by adding or removing a letter.

                nodes[usize::from(rhs)].1.push(lhs);
                nodes[usize::from(lhs)].1.push(rhs);
            }
        }

        // BFS.

        let mut groups = 0_u32;
        let mut max_group_size = 0_u16;
        let mut queue = VecDeque::new();

        for (count, neighbors) in &nodes {
            let count = count.take();
            let mut neighbors = neighbors;

            if count != 0 {
                groups += 1;

                let mut group_size = count;

                loop {
                    for &neighbor in neighbors {
                        let (neighbor_count, neighbor_neighbors) = &nodes[usize::from(neighbor)];
                        let neighbor_count = neighbor_count.take();

                        if neighbor_count != 0 {
                            group_size += neighbor_count;
                            queue.push_back(neighbor_neighbors);
                        }
                    }

                    if let Some(neighbor_neighbors) = queue.pop_front() {
                        neighbors = neighbor_neighbors;
                    } else {
                        break;
                    }
                }

                max_group_size = max_group_size.max(group_size);
            }
        }

        vec![groups as _, i32::from(max_group_size)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn group_strings(words: Vec<String>) -> Vec<i32> {
        Self::group_strings(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
