pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n.cast_unsigned() as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();
        let mut distances = (0..n as u16).collect::<Box<_>>();
        let mut queue = VecDeque::new();

        queries
            .into_iter()
            .map(|query| {
                let [from, to] = query.try_into().ok().unwrap();
                let from = from.cast_unsigned() as usize;
                let to = to.cast_unsigned() as usize;

                graph[from].push(to as u16);

                let mut candidate = distances[from] + 1;
                let to_distance = &mut distances[to];

                if candidate < *to_distance {
                    *to_distance = candidate;

                    queue.push_back(to as u16);

                    loop {
                        candidate += 1;

                        for _ in 0..queue.len() {
                            let node = usize::from(queue.pop_front().unwrap());

                            for &next in &graph[node] {
                                let next_distance = &mut distances[usize::from(next)];

                                if candidate < *next_distance {
                                    *next_distance = candidate;
                                    queue.push_back(next);
                                }
                            }

                            let next = node + 1;

                            if let Some(next_distance) = distances.get_mut(next)
                                && candidate < *next_distance
                            {
                                *next_distance = candidate;
                                queue.push_back(next as u16);
                            }
                        }

                        if queue.is_empty() {
                            break;
                        }
                    }
                }

                i32::from(*distances.last().unwrap())
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::shortest_distance_after_queries(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
