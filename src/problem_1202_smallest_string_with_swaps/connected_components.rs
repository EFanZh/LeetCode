pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let s = s.into_bytes();
        let n = s.len();
        let mut graph = vec![Vec::new(); n];

        for pair in pairs {
            let [left, right]: [_; 2] = pair.try_into().ok().unwrap();
            let left = left as usize;
            let right = right as usize;

            graph[left].push(right);
            graph[right].push(left);
        }

        let mut queue = VecDeque::new();
        let mut components = Vec::new();
        let mut component_ids = vec![usize::MAX; n];

        for node in 0..n {
            let neighbors = mem::take(&mut graph[node]);

            if !neighbors.is_empty() {
                let mut node = node;
                let mut neighbors = neighbors;
                let mut component = Vec::with_capacity(2);
                let component_id = components.len();

                loop {
                    component_ids[node] = component_id;
                    component.push(s[node]);

                    for next in neighbors {
                        let next_neighbors = mem::take(&mut graph[next]);

                        if !next_neighbors.is_empty() {
                            queue.push_back((next, next_neighbors));
                        }
                    }

                    if let Some((next, next_neighbors)) = queue.pop_front() {
                        node = next;
                        neighbors = next_neighbors;
                    } else {
                        break;
                    }
                }

                component.sort_unstable_by_key(|&c| Reverse(c));

                components.push(component);
            }
        }

        drop(graph);
        drop(queue);

        let mut result = s;

        for (target, component) in result.iter_mut().zip(component_ids) {
            if let Some(c) = components.get_mut(component).and_then(Vec::pop) {
                *target = c;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        Self::smallest_string_with_swaps(s, pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
