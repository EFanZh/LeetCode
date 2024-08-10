pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::slice::IterMut;

struct Node {
    value: i32,
    in_degree: u32,
    nexts: Vec<u32>,
}

impl Solution {
    fn dfs(graph: &mut [Node], node: usize, result: &mut IterMut<Vec<i32>>) {
        let value = graph[node].value;

        while let Some(child) = graph[node].nexts.pop() {
            let child = child as usize;
            let child_value = graph[child].value;

            Self::dfs(graph, child, result);

            result.next_back().unwrap().copy_from_slice(&[value, child_value]);
        }
    }

    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pairs = pairs;
        let mut to_interned = HashMap::new();
        let mut graph = Vec::new();

        for pair in &pairs {
            let [from, to] = <[_; 2]>::map(pair.as_slice().try_into().ok().unwrap(), |x| {
                match to_interned.entry(x) {
                    Entry::Occupied(entry) => *entry.get(),
                    Entry::Vacant(entry) => {
                        let interned = graph.len() as u32;

                        entry.insert(interned);

                        graph.push(Node {
                            value: x,
                            in_degree: 0,
                            nexts: Vec::new(),
                        });

                        interned
                    }
                }
            });

            graph[from as usize].nexts.push(to);
            graph[to as usize].in_degree += 1;
        }

        let start = graph
            .iter()
            .enumerate()
            .find_map(|(node, node_state)| (node_state.nexts.len() as u32 > node_state.in_degree).then_some(node))
            .unwrap_or(0);

        Self::dfs(&mut graph, start, &mut pairs.iter_mut());

        pairs
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::valid_arrangement(pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
