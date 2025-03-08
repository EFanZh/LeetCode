pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::slice::IterMut;

impl Solution {
    fn dfs(graph: &mut HashMap<i32, (u32, Vec<i32>)>, node: i32, result: &mut IterMut<Vec<i32>>) {
        while let Some(child) = graph.get_mut(&node).and_then(|(_, nexts)| nexts.pop()) {
            Self::dfs(graph, child, result);

            result.next_back().unwrap().copy_from_slice(&[node, child]);
        }

        graph.remove(&node);
    }

    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pairs = pairs;
        let mut graph = HashMap::<_, (u32, Vec<_>)>::new();

        for pair in &pairs {
            let [from, to] = pair.as_slice().try_into().ok().unwrap();

            match graph.entry(from) {
                Entry::Occupied(entry) => entry.into_mut().1.push(to),
                Entry::Vacant(entry) => {
                    entry.insert((0, vec![to]));
                }
            }

            match graph.entry(to) {
                Entry::Occupied(entry) => entry.into_mut().0 += 1,
                Entry::Vacant(entry) => {
                    entry.insert((1, Vec::new()));
                }
            }
        }

        let start = graph
            .iter()
            .find_map(|(&node, (in_degree, nexts))| (nexts.len() as u32 > *in_degree).then_some(node))
            .unwrap_or_else(|| pairs[0][0]);

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
