pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let darts = darts
            .into_iter()
            .map(|dart| <(_, _)>::from(<[_; 2]>::try_from(dart).ok().unwrap()))
            .collect::<Box<_>>();

        let darts = darts.as_ref();
        let max_distance_squared = r * r * 4;
        let mut angles = Vec::new();
        let mut result = 0;

        for center in darts {
            let mut count = 0;

            // Collect points that is reachable from `center`.

            for point in darts {
                let x_diff = point.0 - center.0;
                let y_diff = point.1 - center.1;
                let distance_squared = x_diff * x_diff + y_diff * y_diff;

                if distance_squared == 0 {
                    count += 1;
                } else if distance_squared <= max_distance_squared {
                    let base = f64::from(y_diff).atan2(f64::from(x_diff));

                    let delta = (f64::from(distance_squared) / f64::from(max_distance_squared))
                        .sqrt()
                        .acos();

                    angles.push((base - delta, false));
                    angles.push((base + delta, true));
                }
            }

            // Angular sweep.

            angles.sort_unstable_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap().then_with(|| lhs.1.cmp(&rhs.1)));

            result = result.max(count);

            for &(_, is_exit) in &angles {
                count += 1 - i32::from(is_exit) * 2;
                result = result.max(count);
            }

            angles.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        Self::num_points(darts, r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
