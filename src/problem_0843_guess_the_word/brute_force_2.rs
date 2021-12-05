use super::Master;
use std::collections::HashSet;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn get_matches(left: &str, right: &str) -> u8 {
        let mut result = 0;

        for (lhs, rhs) in left.bytes().zip(right.bytes()) {
            if lhs == rhs {
                result += 1;
            }
        }

        result
    }

    fn remove_node(graph: &mut HashMap<u8, [HashSet<u8>; 6]>, node: u8) -> [HashSet<u8>; 6] {
        let removed = graph.remove(&node).unwrap();

        for (i, nexts) in removed.iter().enumerate() {
            for next in nexts {
                graph.get_mut(next).unwrap()[i].remove(&node);
            }
        }

        removed
    }

    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        // Build graph.

        let mut graph = HashMap::<u8, [HashSet<u8>; 6]>::new();
        let mut iter = (0..).zip(&words);

        while let Some((i, left)) = iter.next() {
            for (j, right) in iter.clone() {
                let matches = usize::from(Self::get_matches(left, right));

                graph.entry(i).or_default()[matches].insert(j);
                graph.entry(j).or_default()[matches].insert(i);
            }
        }

        // Guess.

        loop {
            let selected = *graph
                .iter()
                .min_by_key(|&(_, edges)| edges.iter().map(HashSet::len).max().unwrap())
                .unwrap()
                .0;

            let matches = master.guess(words[usize::from(selected)].clone()) as usize;

            if matches == 6 {
                break;
            }

            let edges = Self::remove_node(&mut graph, selected);

            for nodes in &edges[..matches] {
                for &node in nodes {
                    Self::remove_node(&mut graph, node);
                }
            }

            for nodes in &edges[matches + 1..] {
                for &node in nodes {
                    Self::remove_node(&mut graph, node);
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_secret_word(words: Vec<String>, master: &Master) {
        Self::find_secret_word(words, master);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
