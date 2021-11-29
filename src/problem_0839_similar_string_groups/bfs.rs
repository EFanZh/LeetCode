pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn is_similar(s: &str, goal: &str) -> bool {
        let mut iter = s.bytes().zip(goal.bytes()).map(|(lhs, rhs)| lhs == rhs);

        while let Some(equal) = iter.next() {
            if !equal {
                loop {
                    if !iter.next().unwrap() {
                        return iter.all(|equal| equal);
                    }
                }
            }
        }

        true
    }

    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut graph = vec![Vec::new(); n];

        for (i, lhs) in strs[..n - 1].iter().enumerate() {
            let mut j = i + 1;

            for rhs in &strs[i + 1..] {
                if Self::is_similar(lhs, rhs) {
                    graph[i].push(j);
                    graph[j].push(i);
                }

                j += 1;
            }
        }

        let mut result = 0;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        for mut node in 0..n {
            if let visited_value @ false = &mut visited[node] {
                *visited_value = true;

                loop {
                    for &next in &graph[node] {
                        if let next_visited_value @ false = &mut visited[next] {
                            *next_visited_value = true;

                            queue.push_back(next);
                        }
                    }

                    if let Some(next_node) = queue.pop_front() {
                        node = next_node;
                    } else {
                        break;
                    }
                }

                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_similar_groups(strs: Vec<String>) -> i32 {
        Self::num_similar_groups(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
