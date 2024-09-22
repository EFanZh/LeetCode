pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node {
    value: u32,
    children: [u16; 4],
    children_distances: [u8; 4],
    children_length: u8,
    distance: u8,
}

struct Item {
    distance: u8,
    node: u16,
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
        other.distance.cmp(&self.distance)
    }
}

struct ContextMut {
    used_nodes: Vec<u16>,
    result: u32,
}

impl Solution {
    fn dfs(nodes: &[Node], context_mut: &mut ContextMut, node: &Node, budget_time: u8, quality: u32) {
        for (&child, &edge_time) in node
            .children
            .iter()
            .zip(&node.children_distances)
            .take(usize::from(node.children_length))
        {
            let child_node = &nodes[usize::from(child)];

            if budget_time >= edge_time + child_node.distance {
                if child == 0 {
                    context_mut.result = context_mut.result.max(quality);
                }

                let (need_to_pop, child_quality) = if context_mut.used_nodes.contains(&child) {
                    (false, quality)
                } else {
                    context_mut.used_nodes.push(child);

                    (true, quality + child_node.value)
                };

                Self::dfs(nodes, context_mut, child_node, budget_time - edge_time, child_quality);

                if need_to_pop {
                    context_mut.used_nodes.pop();
                }
            }
        }
    }

    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let max_time = max_time as u8;

        let mut nodes = values
            .into_iter()
            .map(|value| Node {
                value: value as _,
                children: [0; 4],
                children_distances: [0; 4],
                children_length: 0,
                distance: max_time,
            })
            .collect::<Vec<_>>();

        for edge in edges {
            let [from, to, time] = <[_; 3]>::map(edge.try_into().ok().unwrap(), |x| x as u32);

            for (from, to) in [(from, to), (to, from)] {
                let from_node = &mut nodes[from as usize];

                from_node.children[usize::from(from_node.children_length)] = to as _;
                from_node.children_distances[usize::from(from_node.children_length)] = time as _;
                from_node.children_length += 1;
            }
        }

        nodes[0].distance = 0;

        let mut queue = BinaryHeap::new();
        let mut item = Item { distance: 0, node: 0 };
        let mut children_buffer;
        let mut children_distance_buffer;

        'outer: loop {
            let node = &nodes[usize::from(item.node)];

            children_buffer = node.children;
            children_distance_buffer = node.children_distances;

            for (&child, &time) in children_buffer
                .iter()
                .zip(&children_distance_buffer)
                .take(usize::from(node.children_length))
            {
                let candidate = item.distance + time;

                if candidate < max_time {
                    let child_node = &mut nodes[usize::from(child)];

                    if candidate < child_node.distance {
                        child_node.distance = candidate;

                        queue.push(Item {
                            distance: candidate,
                            node: child,
                        });
                    }
                }
            }

            loop {
                if let Some(next) = queue.pop() {
                    if next.distance == nodes[usize::from(next.node)].distance {
                        item = next;

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        let node = nodes.first().unwrap();

        let mut context_mut = ContextMut {
            used_nodes: vec![0],
            result: node.value,
        };

        Self::dfs(&nodes, &mut context_mut, node, max_time, node.value);

        context_mut.result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        Self::maximal_path_quality(values, edges, max_time)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { distance: 2, node: 3 } == Item { distance: 2, node: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
