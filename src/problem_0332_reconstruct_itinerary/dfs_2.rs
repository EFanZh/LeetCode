pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn dfs<'a>(
        graph: &mut HashMap<String, Vec<String>>,
        node: String,
        result: &mut impl Iterator<Item = &'a mut String>,
    ) {
        while let Some(next) = graph.get_mut(&node).and_then(Vec::pop) {
            Self::dfs(graph, next, result);
        }

        *result.next().unwrap() = node;
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let length = tickets.len() + 1;
        let mut graph = HashMap::new();

        for mut ticket in tickets {
            let to = ticket.pop().unwrap();
            let from = ticket.pop().unwrap();

            match graph.entry(from) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![to]);
                }
                Entry::Occupied(entry) => entry.into_mut().push(to),
            }
        }

        for nexts in graph.values_mut() {
            nexts.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));
        }

        let mut result = vec![String::new(); length];

        Self::dfs(&mut graph, String::from("JFK"), &mut result.iter_mut().rev());

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        Self::find_itinerary(tickets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
