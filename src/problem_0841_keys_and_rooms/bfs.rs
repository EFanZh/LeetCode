pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; rooms.len()];
        let mut node = 0;
        let mut visited_count = 1;

        visited[0] = true;

        loop {
            for &next in &rooms[node as usize] {
                if let visited_value @ false = &mut visited[next as usize] {
                    *visited_value = true;
                    visited_count += 1;

                    queue.push_back(next);
                }
            }

            if let Some(next_node) = queue.pop_front() {
                node = next_node;
            } else {
                break;
            }
        }

        visited_count == rooms.len()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        Self::can_visit_all_rooms(rooms)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
