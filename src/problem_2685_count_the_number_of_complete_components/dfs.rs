pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n.cast_unsigned() as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut seen = vec![false; n].into_boxed_slice();
        let mut stack = Vec::new();
        let mut result = 0;
        let mut group = Vec::new();

        for i in 0..n {
            if let seen_node @ false = &mut seen[i] {
                *seen_node = true;

                let mut node = i;

                loop {
                    for &neighbor in &graph[node] {
                        if let seen_neighbor @ false = &mut seen[neighbor as usize] {
                            *seen_neighbor = true;

                            group.push(neighbor);
                            stack.push(neighbor);
                        }
                    }

                    if let Some(next) = stack.pop() {
                        node = next as _;
                    } else {
                        break;
                    }
                }

                let expected_neighbors = group.len();

                result += i32::from(
                    iter::once(i)
                        .chain(group.iter().copied().map(|node| node as usize))
                        .all(|node| graph[node].len() == expected_neighbors),
                );

                group.clear();
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::count_complete_components(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
