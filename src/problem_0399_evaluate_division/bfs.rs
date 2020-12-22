pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;

impl Solution {
    fn extract_edge(edge: &[String]) -> (&str, &str) {
        let [dividend, divisor]: &[String; 2] = edge.try_into().unwrap();

        (dividend.as_str(), divisor.as_str())
    }

    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph = HashMap::<&str, Vec<(f64, &str)>>::new();

        for ((dividend, divisor), value) in equations.iter().map(|edge| Self::extract_edge(edge)).zip(values) {
            graph
                .entry(divisor)
                .and_modify(|edges| edges.push((value, dividend)))
                .or_insert_with(|| vec![(value, dividend)]);

            graph
                .entry(dividend)
                .and_modify(|edges| edges.push((1.0 / value, divisor)))
                .or_insert_with(|| vec![(1.0 / value, divisor)]);
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queries
            .iter()
            .map(|query| {
                let (dividend, divisor) = Self::extract_edge(query);

                if graph.contains_key(dividend) && graph.contains_key(divisor) {
                    if divisor == dividend {
                        1.0
                    } else {
                        let mut result = 1.0;
                        let mut node = divisor;

                        visited.insert(node);

                        loop {
                            for (value, next) in graph[node].iter().copied() {
                                if next == dividend {
                                    queue.clear();
                                    visited.clear();

                                    return result * value;
                                }

                                if visited.insert(next) {
                                    queue.push_back((result * value, next));
                                }
                            }

                            if let Some((next_result, next_node)) = queue.pop_front() {
                                result = next_result;
                                node = next_node;
                            } else {
                                visited.clear();

                                return -1.0;
                            }
                        }
                    }
                } else {
                    -1.0
                }
            })
            .collect()
    }
}

impl super::Solution for Solution {
    fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        Self::calc_equation(equations, values, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
