pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};

impl Solution {
    fn add_to_graph(graph: &mut HashMap<i32, Vec<usize>>, (index, value): (usize, i32)) {
        graph
            .entry(value)
            .and_modify(|indices| indices.push(index))
            .or_insert_with(|| vec![index]);
    }

    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut result = 0;

        if n != 1 {
            let target = n - 1;
            let mut iter = arr.iter().copied().enumerate();
            let mut graph = HashMap::new();
            let mut prev = iter.next().unwrap();

            'outer: loop {
                if let Some(current) = iter.next() {
                    if current.1 != prev.1 {
                        Self::add_to_graph(&mut graph, prev);
                        Self::add_to_graph(&mut graph, current);

                        prev = current;

                        loop {
                            if let Some(current) = iter.next() {
                                if current.1 == prev.1 {
                                    break;
                                }

                                Self::add_to_graph(&mut graph, current);

                                prev = current;
                            } else {
                                break 'outer;
                            }
                        }
                    }

                    prev = current;
                } else {
                    Self::add_to_graph(&mut graph, prev);

                    break;
                }
            }

            let mut queue = VecDeque::from([0]);
            let mut visited = vec![false; n];

            visited[0] = true;

            'outer: loop {
                result += 1;

                for _ in 0..queue.len() {
                    let current = queue.pop_front().unwrap();

                    if let Some(indices) = graph.remove(&arr[current]) {
                        for index in indices {
                            if index == target {
                                break 'outer;
                            }

                            if let visited_value @ false = &mut visited[index] {
                                *visited_value = true;

                                queue.push_back(index);
                            }
                        }
                    }

                    for index in [current.wrapping_sub(1), current + 1] {
                        if index == target {
                            break 'outer;
                        }

                        if let Some(visited_value @ false) = visited.get_mut(index) {
                            *visited_value = true;

                            queue.push_back(index);
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_jumps(arr: Vec<i32>) -> i32 {
        Self::min_jumps(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
