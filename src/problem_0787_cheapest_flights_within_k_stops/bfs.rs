pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as u32, dst as u32, k as u32);
        let mut graph = vec![Vec::new(); n];

        for flight in &flights {
            let [from, to, price]: [i32; 3] = flight.as_slice().try_into().unwrap();

            graph[from as usize].push((to as u32, price as u32));
        }

        let mut queue = VecDeque::from(vec![src]);
        let mut prices = vec![(u32::MAX, u32::MAX); n];
        let mut visited = HashSet::new();

        prices[src as usize] = (0, 0);

        for _ in 0..=k {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let current_price = prices[node as usize].0;

                for &(next_node, price) in &graph[node as usize] {
                    let next_price = &mut prices[next_node as usize].1;
                    let new_price = current_price + price;

                    if new_price < *next_price {
                        *next_price = new_price;

                        if visited.insert(next_node) {
                            queue.push_back(next_node);
                        }
                    }
                }
            }

            visited.clear();

            for &node in &queue {
                let (current_price, next_price) = &mut prices[node as usize];

                *current_price = *next_price;
            }
        }

        prices[dst as usize].0 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        Self::find_cheapest_price(n, flights, src, dst, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
