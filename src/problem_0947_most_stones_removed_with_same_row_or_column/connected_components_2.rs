pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::mem;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut graph = vec![Vec::new(); n];
        let mut connections = HashMap::with_capacity(n * 2);

        for (i, stone) in stones.iter().enumerate() {
            let [x, y]: [_; 2] = stone.as_slice().try_into().unwrap();

            for key in [x, !y] {
                connections
                    .entry(key)
                    .and_modify(|prev| {
                        let prev = mem::replace(prev, i);

                        graph[prev].push(i);
                        graph[i].push(prev);
                    })
                    .or_insert(i);
            }
        }

        let mut components = 0;
        let mut queue = Vec::new();
        let mut visited = vec![false; n];

        for i in 0..n {
            if let value @ false = &mut visited[i] {
                *value = true;
                components += 1;

                let mut node = i;

                loop {
                    for &next in &graph[node] {
                        if let next_value @ false = &mut visited[next] {
                            *next_value = true;
                            queue.push(next);
                        }
                    }

                    if let Some(next) = queue.pop() {
                        node = next;
                    } else {
                        break;
                    }
                }
            }
        }

        (n - components) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        Self::remove_stones(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
