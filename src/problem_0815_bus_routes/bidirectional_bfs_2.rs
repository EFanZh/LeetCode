pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            0
        } else {
            // Group routes by stops.

            let mut stop_routes = HashMap::<_, Vec<_>>::new();

            for (route, route_stops) in routes.iter().enumerate() {
                for &stop in route_stops {
                    stop_routes
                        .entry(stop)
                        .and_modify(|routes| routes.push(route))
                        .or_insert_with(|| vec![route]);
                }
            }

            // Build graph.

            let mut graph = vec![Vec::new(); routes.len()];

            for routes in stop_routes.values() {
                for (i, &route_1) in (1..).zip(routes) {
                    for &route_2 in &routes[i..] {
                        graph[route_1].push(route_2);
                        graph[route_2].push(route_1);
                    }
                }
            }

            // Bidirectional BFS.

            if let (Some(left_routes), Some(right_routes)) = (stop_routes.get(&source), stop_routes.get(&target)) {
                let mut states = vec![0; routes.len()];
                let mut left_queue = VecDeque::new();
                let mut right_queue = VecDeque::new();

                for &route in left_routes {
                    states[route] = 1;
                    left_queue.push_back(route);
                }

                for &route in right_routes {
                    match &mut states[route] {
                        state @ 0 => {
                            *state = 2;
                            right_queue.push_back(route);
                        }
                        _ => return 1,
                    }
                }

                let mut result = 2;

                loop {
                    if right_queue.len() < left_queue.len() {
                        for _ in 0..right_queue.len() {
                            for &next in &graph[right_queue.pop_front().unwrap()] {
                                match &mut states[next] {
                                    state @ 0 => {
                                        *state = 2;
                                        right_queue.push_front(next);
                                    }
                                    1 => return result,
                                    _ => {}
                                }
                            }
                        }

                        if right_queue.is_empty() {
                            break;
                        }
                    } else {
                        for _ in 0..left_queue.len() {
                            for &next in &graph[left_queue.pop_front().unwrap()] {
                                match &mut states[next] {
                                    state @ 0 => {
                                        *state = 1;
                                        left_queue.push_front(next);
                                    }
                                    2 => return result,
                                    _ => {}
                                }
                            }
                        }

                        if left_queue.is_empty() {
                            break;
                        }
                    }

                    result += 1;
                }
            }

            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        Self::num_buses_to_destination(routes, source, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
