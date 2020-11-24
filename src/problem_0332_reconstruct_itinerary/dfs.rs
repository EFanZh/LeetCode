pub struct Solution;

use std::cmp::Reverse;
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    fn name_to_id(name: &str) -> u32 {
        let [a, b, c]: [u8; 3] = name.as_bytes().try_into().unwrap();

        u32::from_le_bytes([c, b, a, 0])
    }

    fn id_to_name(id: u32) -> String {
        let [c, b, a, _] = id.to_le_bytes();

        String::from_utf8(vec![a, b, c]).unwrap()
    }

    fn dfs<'a>(graph: &mut HashMap<u32, Vec<u32>>, node: u32, result: &mut impl Iterator<Item = &'a mut String>) {
        while let Some(next) = graph.get_mut(&node).and_then(Vec::pop) {
            Self::dfs(graph, next, result);
        }

        *result.next().unwrap() = Self::id_to_name(node);
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let length = tickets.len() + 1;
        let mut graph = HashMap::<u32, Vec<u32>>::new();

        for ticket in tickets {
            let [from, to]: &[String; 2] = ticket.as_slice().try_into().unwrap();
            let from = Self::name_to_id(from);
            let to = Self::name_to_id(to);

            graph
                .entry(from)
                .and_modify(|nexts| nexts.push(to))
                .or_insert_with(|| vec![to]);
        }

        for nexts in graph.values_mut() {
            nexts.sort_unstable_by_key(|&next| Reverse(next));
        }

        let mut result = vec![String::new(); length];

        Self::dfs(&mut graph, Self::name_to_id("JFK"), &mut result.iter_mut().rev());

        result
    }
}

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
