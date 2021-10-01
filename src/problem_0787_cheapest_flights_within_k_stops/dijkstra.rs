pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

struct Item {
    node: u32,
    steps: u32,
    price: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.price, &self.price)
    }
}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as u32, dst as u32, k as u32);
        let mut graph = vec![Vec::new(); n];

        for flight in &flights {
            let [from, to, price]: [i32; 3] = flight.as_slice().try_into().unwrap();

            graph[from as usize].push((to as u32, price as u32));
        }

        let mut queue = BinaryHeap::new();
        let mut distances = HashMap::new();

        let mut item = Item {
            node: src,
            steps: k + 1,
            price: 0,
        };

        loop {
            if item.node == dst {
                return item.price as _;
            }

            if let Some(next_steps) = item.steps.checked_sub(1) {
                for &(next_node, price) in &graph[item.node as usize] {
                    let next_price = item.price + price;

                    match distances.entry((next_node, next_steps)) {
                        Entry::Occupied(entry) => {
                            if next_price < *entry.get() {
                                *entry.into_mut() = next_price;
                            } else {
                                continue;
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(next_price);
                        }
                    }

                    queue.push(Item {
                        node: next_node,
                        steps: next_steps,
                        price: next_price,
                    });
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if distances.get(&(next_item.node, next_item.steps)).copied() == Some(next_item.price) {
                        item = next_item;

                        break;
                    }
                } else {
                    return -1;
                }
            }
        }
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
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                node: 2,
                steps: 3,
                price: 5
            } == Item {
                node: 7,
                steps: 11,
                price: 5
            }
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
