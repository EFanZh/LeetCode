pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as u32 as usize;
        let mut nodes = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to] = edge.try_into().ok().unwrap();

            nodes[from as u32 as usize].push(to as u32);
            nodes[to as u32 as usize].push(from as u32);
        }

        let mut total_count = 0_u64;
        let mut result = 0;
        let mut visited = vec![false; n].into_boxed_slice();
        let mut queue = VecDeque::new();

        for mut node in 0..n {
            if let state @ false = &mut visited[node] {
                *state = true;

                let mut count = 1;

                loop {
                    for &next in &nodes[node] {
                        if let state @ false = &mut visited[next as usize] {
                            *state = true;

                            queue.push_back(next);
                        }
                    }

                    if let Some(next) = queue.pop_front() {
                        node = next as _;
                        count += 1;
                    } else {
                        break;
                    }
                }

                result += total_count * count;
                total_count += count;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        Self::count_pairs(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
