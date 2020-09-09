pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let next_y = x % y;

            x = y;
            y = next_y;
        }

        x
    }

    fn get_line_key([x_1, y_1]: [i32; 2], [x_2, y_2]: [i32; 2]) -> (i32, i32) {
        let d_x = x_2 - x_1;
        let d_y = y_2 - y_1;
        let g = Self::gcd(d_x, d_y);

        if g == 0 {
            (0, 0)
        } else {
            let r_x = d_x / g;
            let r_y = d_y / g;

            if r_x < 0 {
                (-r_x, -r_y)
            } else {
                (r_x, r_y)
            }
        }
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut counts = HashMap::new();

        for (i, point_1) in points.iter().map(|p| p.as_slice().try_into().unwrap()).enumerate() {
            let mut base_count = 1;

            for point_2 in points[i + 1..].iter().map(|p| p.as_slice().try_into().unwrap()) {
                let key = Self::get_line_key(point_1, point_2);

                if key == (0, 0) {
                    base_count += 1;
                } else {
                    match counts.entry(key) {
                        Entry::Occupied(entry) => *entry.into_mut() += 1,
                        Entry::Vacant(entry) => {
                            entry.insert(1);
                        }
                    }
                }
            }

            result = result.max(base_count + counts.drain().map(|(_, v)| v).max().unwrap_or(0));
        }

        result
    }
}

impl super::Solution for Solution {
    fn max_points(points: Vec<Vec<i32>>) -> i32 {
        Self::max_points(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
