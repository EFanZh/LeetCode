pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut distances = vec![u32::MAX; n * 2].into_boxed_slice();
        let (distances_1, distances_2) = distances.split_at_mut(n);

        for (node, distances) in [(node1, &mut *distances_1), (node2, &mut *distances_2)] {
            let mut node = node as u32 as usize;
            let mut distance = 0;

            while let Some(state @ &mut u32::MAX) = distances.get_mut(node) {
                *state = distance;
                distance += 1;
                node = edges[node] as _;
            }
        }

        let mut result = -1;
        let mut min_distance = u32::MAX;

        (0..)
            .zip(distances_1.iter().zip(&*distances_2))
            .for_each(|(node, (&distance_1, &distance_2))| {
                let distance = distance_1.max(distance_2);

                if distance < min_distance {
                    min_distance = distance;
                    result = node;
                }
            });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        Self::closest_meeting_node(edges, node1, node2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
