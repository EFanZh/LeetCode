pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let points = points
            .into_iter()
            .map(Vec::try_into)
            .collect::<Result<Box<[[_; 2]]>, _>>()
            .ok()
            .unwrap();

        let n = points.len();
        let mut result = 0;

        let mut distances = vec![u32::MAX; n].into_boxed_slice();

        distances[0] = 0;

        let mut current = points[0];

        for _ in 1..n {
            let mut best_index = 0;
            let mut best_point = [0, 0];
            let mut best_distance = u32::MAX;

            for (next_index, (&next, current_distance)) in points.iter().zip(&mut *distances).enumerate() {
                if *current_distance != 0 {
                    let [current_x, current_y] = current;
                    let [next_x, next_y] = next;
                    let distance = (next_x - current_x).unsigned_abs() + (next_y - current_y).unsigned_abs();

                    if distance < *current_distance {
                        *current_distance = distance;
                    }

                    if *current_distance < best_distance {
                        best_index = next_index;
                        best_point = next;
                        best_distance = *current_distance;
                    }
                }
            }

            distances[best_index] = 0;
            result += best_distance;
            current = best_point;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        Self::min_cost_connect_points(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
