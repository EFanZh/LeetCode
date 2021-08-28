pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::mem;

const D: usize = 2;

impl Solution {
    fn heap_relax(heap: &mut Vec<usize>, node_attributes: &mut [(usize, u32)], node: usize, distance: u32) {
        let (mut index, current_distance) = &mut node_attributes[node];

        if distance < *current_distance {
            *current_distance = distance;

            if index == usize::MAX {
                index = heap.len();

                heap.push(0);
            }

            while index != 0 {
                let parent_index = (index - 1) / D;
                let parent_node = heap[parent_index];
                let parent_distance = node_attributes[parent_node].1;

                if distance < parent_distance {
                    heap[index] = parent_node;
                    node_attributes[parent_node].0 = index;
                    index = parent_index;
                } else {
                    break;
                }
            }

            heap[index] = node;
            node_attributes[node].0 = index;
        }
    }

    fn heap_pop(heap: &mut Vec<usize>, node_attributes: &mut [(usize, u32)]) -> Option<(usize, u32)> {
        heap.pop().map(|mut result_node| {
            if let Some(&first) = heap.first() {
                let node = mem::replace(&mut result_node, first);
                let distance = node_attributes[node].1;
                let mut index = 0;

                while let Some((child_node, (child_index, child_distance))) = heap
                    .iter()
                    .skip(index * D + 1)
                    .take(D)
                    .map(|&node| (node, node_attributes[node]))
                    .min_by_key(|&(_, (_, distance))| distance)
                {
                    if child_distance < distance {
                        heap[index] = child_node;
                        node_attributes[child_node].0 = index;
                        index = child_index;
                    } else {
                        break;
                    }
                }

                heap[index] = node;
                node_attributes[node].0 = index;
            }

            (result_node, node_attributes[result_node].1)
        })
    }

    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as _;
        let mut graph = vec![Vec::new(); n];

        for edge in times {
            let [from, to, weight]: [i32; 3] = edge.as_slice().try_into().unwrap();

            graph[(from - 1) as usize].push(((to - 1) as usize, weight as u32));
        }

        let mut item = ((k - 1) as usize, 0);
        let mut heap = Vec::with_capacity(n);
        let mut node_attributes = vec![(usize::MAX, u32::MAX); n];

        node_attributes[item.0].1 = 0;

        loop {
            for &(next_node, weight) in &graph[item.0] {
                Self::heap_relax(&mut heap, &mut node_attributes, next_node, item.1 + weight);
            }

            if let Some(next_item) = Self::heap_pop(&mut heap, &mut node_attributes) {
                item = next_item;
            } else {
                break;
            }
        }

        node_attributes.iter().map(|&(_, distance)| distance).max().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        Self::network_delay_time(times, n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
