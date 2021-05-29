pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut groups = HashMap::new();
        let mut queue = VecDeque::new();

        for mut node in 0..graph.len() as i32 {
            if let Entry::Vacant(entry) = groups.entry(node) {
                entry.insert(false);

                loop {
                    let group = groups[&node];

                    for &next_node in &graph[node as usize] {
                        match groups.entry(next_node) {
                            Entry::Occupied(entry) => {
                                if *entry.get() == group {
                                    return false;
                                }
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(!group);
                                queue.push_back(next_node);
                            }
                        }
                    }

                    if let Some(next_i) = queue.pop_front() {
                        node = next_i;
                    } else {
                        break;
                    }
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        Self::is_bipartite(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
