pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let _ = n;
        let mut graph = HashMap::<_, HashSet<_>>::new();

        for edge in edges {
            let [from, to] = edge.try_into().ok().unwrap();
            let from = from as u16;
            let to = to as u16;

            for (from, to) in [(from, to), (to, from)] {
                match graph.entry(from) {
                    Entry::Occupied(entry) => {
                        entry.into_mut().insert(to);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(HashSet::from([to]));
                    }
                }
            }
        }

        let mut result = usize::MAX / 2;

        for x_neighbors in graph.values() {
            if x_neighbors.len() < result + 2 {
                for y in x_neighbors {
                    let y_neighbors = &graph[y];

                    if x_neighbors.len() + y_neighbors.len() < result + 4 {
                        for z in y_neighbors {
                            if x_neighbors.contains(z) {
                                result = result.min(x_neighbors.len() + y_neighbors.len() + graph[z].len() - 6);
                            }
                        }
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::min_trio_degree(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
