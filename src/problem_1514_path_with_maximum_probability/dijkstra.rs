pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    node: usize,
    probability: f64,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.partial_cmp(&other.probability).unwrap()
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let n = n as u32 as usize;
        let start = start as u32 as usize;
        let end = end as u32 as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for (edge, probability) in edges.into_iter().zip(succ_prob) {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from as u32 as usize;
            let to = to as u32 as usize;

            graph[from].push((to, probability));
            graph[to].push((from, probability));
        }

        let mut probabilities = vec![0.0; n].into_boxed_slice();

        probabilities[start] = 1.0;

        let mut queue = BinaryHeap::new();

        let mut item = Item {
            node: start,
            probability: 1.0,
        };

        loop {
            for &(neighbor, probability) in &graph[item.node] {
                let candidate_probability = item.probability * probability;
                let current_probability = &mut probabilities[neighbor];

                if candidate_probability > *current_probability {
                    *current_probability = candidate_probability;

                    queue.push(Item {
                        node: neighbor,
                        probability: candidate_probability,
                    });
                }
            }

            loop {
                if let Some(next_item) = queue.pop() {
                    if next_item.probability >= probabilities[next_item.node] {
                        if next_item.node == end {
                            return next_item.probability;
                        }

                        item = next_item;

                        break;
                    }
                } else {
                    return 0.0;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        Self::max_probability(n, edges, succ_prob, start, end)
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
                probability: 5.0,
            } == Item {
                node: 3,
                probability: 5.0,
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
