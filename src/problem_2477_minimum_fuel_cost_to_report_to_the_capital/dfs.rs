pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn dfs(graph: &[Vec<u32>], node: u32, parent: u32, seats: NonZeroU32, result: &mut u64) -> u32 {
        let mut node_count = 1;

        for &child in &graph[node as usize] {
            if child != parent {
                let child_node_count = Self::dfs(graph, child, node, seats, result);

                *result += u64::from((child_node_count + seats.get() - 1) / seats);
                node_count += child_node_count;
            }
        }

        node_count
    }

    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let seats = NonZeroU32::new(seats as _).unwrap();
        let mut graph = vec![Vec::new(); roads.len() + 1].into_boxed_slice();

        for road in roads {
            let [from, to] = <[_; 2]>::map(road.try_into().ok().unwrap(), |node| node as u32);

            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut result = 0;

        Self::dfs(&graph, 0, u32::MAX, seats, &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        Self::minimum_fuel_cost(roads, seats)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
