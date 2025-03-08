pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

struct Queue {
    heap: Vec<usize>,
    probabilities: Box<[f64]>,
    indices: Box<[usize]>,
}

impl Queue {
    fn new(n: usize) -> Self {
        Self {
            heap: Vec::new(),
            probabilities: vec![0.0; n].into_boxed_slice(),
            indices: vec![usize::MAX; n].into_boxed_slice(),
        }
    }

    fn sift_up(&mut self, mut index: usize, node: usize, probability: f64) {
        loop {
            let parent_index = index.wrapping_sub(1) / 2;

            if let Some(&parent_node) = self.heap.get(parent_index) {
                if probability > self.probabilities[parent_node] {
                    self.heap[index] = parent_node;
                    self.indices[parent_node] = index;
                    index = parent_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.heap[index] = node;
        self.indices[node] = index;
    }

    fn sift_down(&mut self, mut index: usize, node: usize, probability: f64) {
        loop {
            let mut child_index = index * 2 + 1;

            if let Some(mut child_node) = self.heap.get(child_index).copied() {
                let mut child_probability = self.probabilities[child_node];
                let right_index = child_index + 1;

                if let Some(&right_node) = self.heap.get(right_index) {
                    let right_probability = self.probabilities[right_node];

                    if right_probability > child_probability {
                        child_index = right_index;
                        child_node = right_node;
                        child_probability = right_probability;
                    }
                }

                if child_probability > probability {
                    self.heap[index] = child_node;
                    self.indices[child_node] = index;
                    index = child_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.heap[index] = node;
        self.indices[node] = index;
    }

    fn push(&mut self, node: usize, probability: f64) {
        let current_probability = &mut self.probabilities[node];

        if probability > *current_probability {
            *current_probability = probability;

            let mut index = self.indices[node];

            if index == usize::MAX {
                index = self.heap.len();

                self.heap.push(0); // The value here does not matter.
            };

            self.sift_up(index, node, probability);
        }
    }

    fn pop(&mut self) -> Option<(usize, f64)> {
        self.heap.pop().map(|mut candidate| {
            if let Some(&result) = self.heap.first() {
                let node = mem::replace(&mut candidate, result);

                self.sift_down(0, node, self.probabilities[node]);
            }

            (candidate, self.probabilities[candidate])
        })
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let n = n as u32 as usize;
        let start = start as u32 as usize;
        let end = end as u32 as usize;
        let mut graph = vec![Vec::new(); n];

        for (edge, probability) in edges.into_iter().zip(succ_prob) {
            let [from, to] = edge.try_into().ok().unwrap();
            let from = from as u32 as usize;
            let to = to as u32 as usize;

            graph[from].push((to, probability));
            graph[to].push((from, probability));
        }

        let mut queue = Queue::new(n);
        let mut item = (start, 1.0);

        queue.probabilities[start] = 1.0;

        loop {
            for &(neighbor, probability) in &graph[item.0] {
                queue.push(neighbor, item.1 * probability);
            }

            if let Some(next_item) = queue.pop() {
                if next_item.0 == end {
                    return next_item.1;
                }

                item = next_item;
            } else {
                return 0.0;
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
