pub mod iterative;

pub trait Solution {
    fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 2, 3, -1] as &[_], 0, 1), 2), ((&[1, 2, -1], 0, 2), 2)];

        for ((edges, node1, node2), expected) in test_cases {
            assert_eq!(S::closest_meeting_node(edges.to_vec(), node1, node2), expected);
        }
    }
}
