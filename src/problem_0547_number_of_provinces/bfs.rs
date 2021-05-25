pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut visited = vec![false; is_connected.len()];
        let mut queue = VecDeque::new();

        for mut node in 0..is_connected.len() {
            if let v @ false = &mut visited[node] {
                *v = true;

                result += 1;

                loop {
                    for next in
                        is_connected[node]
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &connected)| if connected == 0 { None } else { Some(i) })
                    {
                        if let v @ false = &mut visited[next] {
                            *v = true;

                            queue.push_back(next);
                        }
                    }

                    if let Some(next_node) = queue.pop_front() {
                        node = next_node;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        Self::find_circle_num(is_connected)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
